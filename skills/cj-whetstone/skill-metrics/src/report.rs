//! The report: one static HTML page from the database, with every style inline and no URL, so it
//! opens offline.

use crate::store::ReportCall;
use crate::version::UNCOMMITTED;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;

/// z for a 95 % interval.
const Z: f64 = 1.96;

/// Wilson score interval of `hits` out of `n`, as (low, high) shares; `n` > 0.
fn wilson(hits: i64, n: i64) -> (f64, f64) {
    let (k, n) = (hits as f64, n as f64);
    let p = k / n;
    let z2 = Z * Z;
    let denominator = 1.0 + z2 / n;
    let center = (p + z2 / (2.0 * n)) / denominator;
    let margin = Z * (p * (1.0 - p) / n + z2 / (4.0 * n * n)).sqrt() / denominator;
    ((center - margin).max(0.0), (center + margin).min(1.0))
}

/// `33 % (6–79 %)`, or `–` with nothing to count.
fn rate(hits: i64, n: i64) -> String {
    if n == 0 {
        return "–".into();
    }
    let percent = |share: f64| (share * 100.0).round() as i64;
    let (low, high) = wilson(hits, n);
    let share = percent(hits as f64 / n as f64);
    format!("{share} % ({}–{} %)", percent(low), percent(high))
}

/// The five rates: name and the call's flag, `None` where its parser did not measure it.
type Flag = fn(&ReportCall) -> Option<bool>;
const CORRECTION: Flag = |c| c.corrected;
const SERIES: [(&str, Flag); 5] = [
    ("Correction", CORRECTION),
    ("Interrupt", |c| c.interrupted),
    ("Denial", |c| c.denied),
    ("Tool error", |c| c.tool_error),
    ("Bare question", |c| c.bare_question),
];

/// Calls where the event happened, and the calls that measured it.
fn hits(calls: &[&ReportCall], flag: Flag) -> (i64, i64) {
    calls
        .iter()
        .filter_map(|&c| flag(c))
        .fold((0, 0), |(hits, n), hit| (hits + i64::from(hit), n + 1))
}

/// The middle value, the mean of the two middle ones for an even count.
fn median(mut values: Vec<i64>) -> Option<i64> {
    values.sort_unstable();
    let mid = values.len() / 2;
    match values.len() {
        0 => None,
        len if len % 2 == 0 => Some((values[mid - 1] + values[mid]) / 2),
        _ => Some(values[mid]),
    }
}

fn tokens(value: i64) -> String {
    let thousands = (value as f64 / 1e3).round();
    match value {
        // Rounded first, so 999_600 reads 1.0M, not 1000k.
        _ if thousands >= 1000.0 => format!("{:.1}M", value as f64 / 1e6),
        v if v >= 10_000 => format!("{thousands}k"),
        v => v.to_string(),
    }
}

fn duration(ms: i64) -> String {
    let seconds = (ms as f64 / 1000.0).round() as i64;
    if seconds < 60 {
        format!("{seconds} s")
    } else {
        format!("{} min", (seconds as f64 / 60.0).round())
    }
}

fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

const HEADERS: [&str; 10] = [
    "Skill",
    "Calls",
    "Correction",
    "Interrupt",
    "Denial",
    "Tool error",
    "Bare question",
    "Recommended",
    "Median tokens",
    "Median active",
];

/// Calls per skill, most first, equal counts by name. With `kind`, only calls of that start kind
/// count, and a skill left with none keeps its entry.
fn by_skill<'a>(calls: &'a [ReportCall], kind: Option<&str>) -> Vec<Skill<'a>> {
    let mut by_skill: BTreeMap<(&str, bool), Vec<&ReportCall>> = BTreeMap::new();
    for call in calls {
        let skill = by_skill.entry((&call.skill, call.nested)).or_default();
        if kind.is_none_or(|kind| call.start_kind.as_deref() == Some(kind)) {
            skill.push(call);
        }
    }
    let mut skills: Vec<_> = by_skill.into_iter().collect();
    skills.sort_by_key(|(_, calls)| std::cmp::Reverse(calls.len()));
    skills
}

/// (skill, nested) and its calls.
type Skill<'a> = ((&'a str, bool), Vec<&'a ReportCall>);

/// The skill's name, marked when it only ever ran nested.
fn name((skill, nested): (&str, bool)) -> String {
    let mark = if nested { " (nested)" } else { "" };
    format!("{}{mark}", escape(skill))
}

/// Correction, interrupt, denial, tool-error and bare-question rate, then the recommended share.
fn rates(calls: &[&ReportCall]) -> [String; 6] {
    // Questions and recommended answers of the calls that measured both.
    let (questions, recommended) = calls
        .iter()
        .filter_map(|c| Some((c.questions?, c.recommended_answers?)))
        .fold((0, 0), |(q, r), (cq, cr)| (q + cq, r + cr));
    let [a, b, c, d, e] = SERIES.map(|(_, flag)| {
        let (hits, n) = hits(calls, flag);
        rate(hits, n)
    });
    [a, b, c, d, e, rate(recommended, questions)]
}

/// Calls, the five rates and the recommended share: the columns overview and versions share.
fn counted(calls: &[&ReportCall]) -> Vec<String> {
    let mut cells = vec![calls.len().to_string()];
    cells.extend(rates(calls));
    cells
}

/// One overview row per skill. A skill that only ever ran nested is marked so; its calls are the
/// windows it was loaded in.
fn overview(skills: &[Skill]) -> String {
    let mut rows = String::new();
    for (skill, calls) in skills {
        let _ = write!(rows, "<tr><th scope=\"row\">{}</th>", name(*skill));
        let cells = counted(calls).into_iter().chain([
            median(calls.iter().filter_map(|c| c.tokens).collect()).map_or("–".into(), tokens),
            median(calls.iter().filter_map(|c| c.active_ms).collect()).map_or("–".into(), duration),
        ]);
        for cell in cells {
            let _ = write!(rows, "<td>{cell}</td>");
        }
        rows.push_str("</tr>\n");
    }
    let head: String = HEADERS
        .iter()
        .map(|header| format!("<th scope=\"col\">{header}</th>"))
        .collect();
    format!(
        "<div class=\"scroll\"><table>\n<thead><tr>{head}</tr></thead>\n<tbody>\n{rows}</tbody>\n\
         </table></div>"
    )
}

/// Commit time and day of a skill version, as the repo has them.
pub type Dates = HashMap<String, (i64, String)>;

/// Where a version's column goes: commits by date (one the repo no longer has after those),
/// then `uncommitted`, then calls without a version.
fn column_order<'a>(version: Option<&'a str>, dates: &'a Dates) -> (u8, Option<i64>, &'a str) {
    match version {
        Some(UNCOMMITTED) => (2, None, ""),
        None => (3, None, ""),
        Some(commit) => match dates.get(commit) {
            Some((time, _)) => (0, Some(*time), commit),
            None => (1, None, commit),
        },
    }
}

fn column_head(version: Option<&str>, dates: &Dates) -> String {
    match version {
        Some(UNCOMMITTED) => "uncommitted".into(),
        None => "no version".into(),
        Some(commit) => {
            let short = escape(commit.get(..7).unwrap_or(commit));
            match dates.get(commit) {
                Some((_, day)) => format!("{short}<br>{}", escape(day)),
                None => short,
            }
        }
    }
}

/// Per skill with calls, one column per skill version side by side and the weekly trend, folded
/// away by default.
fn per_skill(skills: &[Skill], dates: &Dates) -> String {
    let mut page = String::new();
    for (skill, calls) in skills.iter().filter(|(_, calls)| !calls.is_empty()) {
        let mut columns: BTreeMap<_, (Option<&str>, Vec<&ReportCall>)> = BTreeMap::new();
        for call in calls {
            let version = call.skill_version.as_deref();
            columns
                .entry(column_order(version, dates))
                .or_insert_with(|| (version, Vec::new()))
                .1
                .push(call);
        }
        let head: String = columns
            .values()
            .map(|(version, _)| format!("<th scope=\"col\">{}</th>", column_head(*version, dates)))
            .collect();
        let cells: Vec<_> = columns.values().map(|(_, calls)| counted(calls)).collect();
        let mut rows = String::new();
        for (i, label) in HEADERS[1..8].iter().enumerate() {
            let _ = write!(rows, "<tr><th scope=\"row\">{label}</th>");
            for column in &cells {
                let _ = write!(rows, "<td>{}</td>", column[i]);
            }
            rows.push_str("</tr>\n");
        }
        let _ = write!(
            page,
            "<details><summary>{} · {}</summary>\n<div class=\"scroll\"><table>\n\
             <thead><tr><th scope=\"col\">Version</th>{head}</tr></thead>\n<tbody>\n{rows}\
             </tbody>\n</table></div>\n{}</details>\n",
            name(*skill),
            counted_as(calls.len() as i64, skill.1),
            trend(calls, skill.1)
        );
    }
    page
}

/// Number formatting for SVG coordinates.
fn coord(value: f64) -> String {
    format!("{value:.1}")
}

/// Grid lines and labels at 0, 50 and 100 % from `left` to `right`.
fn percent_axis(svg: &mut String, left: f64, right: f64, y: &dyn Fn(f64) -> f64) {
    for share in [0.0, 0.5, 1.0] {
        let _ = write!(
            svg,
            "<line class=\"grid\" x1=\"{left}\" x2=\"{right}\" y1=\"{y}\" y2=\"{y}\"/>\
             <text x=\"{}\" y=\"{}\" text-anchor=\"end\">{} %</text>",
            left - 6.0,
            coord(y(share) + 4.0),
            share * 100.0,
            y = coord(y(share)),
        );
    }
}

/// Label heights for line ends sorted top down: at least `gap` apart, pushed down from the top,
/// then up so none sits below `floor`.
fn spread(ends: &[f64], gap: f64, floor: f64) -> Vec<f64> {
    let mut ys: Vec<f64> = Vec::new();
    for &end in ends {
        let below = ys.last().map_or(f64::MIN, |y| y + gap);
        ys.push(end.max(below));
    }
    let mut floor = floor;
    for y in ys.iter_mut().rev() {
        *y = y.min(floor);
        floor = *y - gap;
    }
    ys
}

/// "1 call", "3 calls", or windows for a skill that only ran nested.
fn counted_as(n: i64, nested: bool) -> String {
    let unit = if nested { "window" } else { "call" };
    let plural = if n == 1 { "" } else { "s" };
    format!("{n} {unit}{plural}")
}

/// One line per rate, one point per ISO week that measured it, 0 to 100 %. Each line ends in a
/// label, nudged apart, so no rate is told by its color alone.
/// A call whose start time does not parse has no week and stays out.
fn trend(calls: &[&ReportCall], nested: bool) -> String {
    let mut weeks: BTreeMap<(i64, &str), Vec<&ReportCall>> = BTreeMap::new();
    for &call in calls {
        if let (Some(monday), Some(week)) = (call.week_monday, &call.week) {
            weeks.entry((monday, week)).or_default().push(call);
        }
    }
    if weeks.is_empty() {
        return String::new();
    }
    let (width, height) = (360.0, 180.0);
    let (left, right, top, bottom) = (44.0, 116.0, 10.0, 24.0);
    let (plot_w, plot_h) = (width - left - right, height - top - bottom);
    let names: Vec<(i64, &str)> = weeks.keys().copied().collect();
    let (first, last) = (names[0], names[names.len() - 1]);
    // Days from the first week's Monday, so a week without calls leaves a gap.
    let x = |monday: i64| match last.0 - first.0 {
        0 => left + plot_w / 2.0,
        span => left + plot_w * (monday - first.0) as f64 / span as f64,
    };
    let y = |share: f64| top + plot_h * (1.0 - share);
    let mut svg = format!(
        "<svg viewBox=\"0 0 {width} {height}\" role=\"img\" aria-label=\"Rates per ISO week\">"
    );
    percent_axis(&mut svg, left, width - right, &y);
    let anchor = if first == last { "middle" } else { "start" };
    let _ = write!(
        svg,
        "<text x=\"{}\" y=\"{}\" text-anchor=\"{anchor}\">{}</text>",
        coord(x(first.0)),
        height - 6.0,
        escape(first.1)
    );
    if first != last {
        let _ = write!(
            svg,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"end\">{}</text>",
            coord(x(last.0)),
            height - 6.0,
            escape(last.1)
        );
    }
    let mut ends = Vec::new();
    for (i, (label, flag)) in SERIES.iter().enumerate() {
        let points: Vec<(f64, f64, String)> = weeks
            .iter()
            .filter_map(|((monday, week), calls)| {
                let (hits, n) = hits(calls, *flag);
                (n > 0).then(|| {
                    let title = format!(
                        "{label}, {week}: {} of {}",
                        rate(hits, n),
                        counted_as(n, nested)
                    );
                    (x(*monday), y(hits as f64 / n as f64), title)
                })
            })
            .collect();
        let Some(&(_, end, _)) = points.last() else {
            continue;
        };
        let line: Vec<String> = points
            .iter()
            .map(|(px, py, _)| format!("{},{}", coord(*px), coord(*py)))
            .collect();
        let _ = write!(
            svg,
            "<polyline class=\"s{}\" points=\"{}\"/>",
            i + 1,
            line.join(" ")
        );
        for (px, py, title) in &points {
            let _ = write!(
                svg,
                "<circle class=\"s{}\" cx=\"{}\" cy=\"{}\" r=\"4\"><title>{}</title></circle>",
                i + 1,
                coord(*px),
                coord(*py),
                escape(title)
            );
        }
        ends.push((end, i + 1, *label));
    }
    // Labels at least a line apart: pushed down from the top, then up from the chart's bottom.
    ends.sort_by(|a, b| a.0.total_cmp(&b.0));
    let ys: Vec<f64> = ends.iter().map(|(end, ..)| *end).collect();
    let label_ys = spread(&ys, 13.0, height - 4.0);
    for ((_, series, label), label_y) in ends.into_iter().zip(label_ys) {
        let _ = write!(
            svg,
            "<line class=\"s{series}\" x1=\"{}\" x2=\"{}\" y1=\"{y}\" y2=\"{y}\"/>\
             <text x=\"{}\" y=\"{}\">{label}</text>",
            width - right + 6.0,
            width - right + 16.0,
            width - right + 20.0,
            coord(label_y + 4.0),
            y = coord(label_y),
        );
    }
    svg.push_str("</svg>\n");
    svg
}

/// One point per skill with calls of its own: calls on a log axis against correction rate. Most
/// calls first, a point gets a label where it overlaps no earlier one; every point names itself
/// on hover.
fn scatter(skills: &[Skill]) -> String {
    // Skills with the same calls and counts share one point, named together, so none hides
    // behind another and the interval in its title holds for each.
    let mut dots: Vec<(String, usize, (i64, i64))> = Vec::new();
    for ((skill, nested), calls) in skills {
        let (hits, n) = hits(calls, CORRECTION);
        if *nested || n == 0 {
            continue;
        }
        let same = |(_, c, (h, m)): &&mut (String, usize, (i64, i64))| {
            *c == calls.len() && (*h, *m) == (hits, n)
        };
        match dots.iter_mut().find(same) {
            Some(dot) => dot.0 = format!("{}, {skill}", dot.0),
            None => dots.push(((*skill).to_owned(), calls.len(), (hits, n))),
        }
    }
    let Some(most) = dots.iter().map(|(_, calls, _)| *calls).max() else {
        return "<p>No skill call measured corrections.</p>\n".into();
    };
    let decades = (most as f64).log10().ceil().max(1.0) as i32;
    let (width, height) = (400.0, 250.0);
    let (left, right, top, bottom) = (44.0, 16.0, 24.0, 38.0);
    let (plot_w, plot_h) = (width - left - right, height - top - bottom);
    let x = |calls: f64| left + plot_w * calls.log10() / f64::from(decades);
    let y = |share: f64| top + plot_h * (1.0 - share);
    let mut svg = format!(
        "<svg viewBox=\"0 0 {width} {height}\" role=\"img\" \
         aria-label=\"Calls against correction rate per skill\">"
    );
    percent_axis(&mut svg, left, width - right, &y);
    for decade in 0..=decades {
        let calls = 10f64.powi(decade);
        let _ = write!(
            svg,
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{calls}</text>",
            coord(x(calls)),
            height - 20.0
        );
    }
    let _ = write!(
        svg,
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">Calls (log scale)</text>",
        coord(left + plot_w / 2.0),
        height - 4.0
    );
    // Every point is an obstacle for labels, so no label covers a point.
    let points: Vec<(f64, f64, f64, f64)> = dots
        .iter()
        .map(|(_, calls, (hits, n))| {
            let (px, py) = (x(*calls as f64), y(*hits as f64 / *n as f64));
            (px - 4.0, py - 4.0, px + 4.0, py + 4.0)
        })
        .collect();
    let mut placed: Vec<(f64, f64, f64, f64)> = Vec::new();
    for (skill, calls, (hits, n)) in &dots {
        let (px, py) = (x(*calls as f64), y(*hits as f64 / *n as f64));
        let _ = write!(
            svg,
            "<circle class=\"dot\" cx=\"{}\" cy=\"{}\" r=\"4\"><title>{}: {}, correction \
             {}</title></circle>",
            coord(px),
            coord(py),
            escape(skill),
            counted_as(*calls as i64, false),
            rate(*hits, *n)
        );
        // Several skills at one point: the first name and how many more.
        let more = skill.matches(", ").count();
        let skill = match skill.split_once(", ") {
            Some((first, _)) => format!("{first} +{more}"),
            None => skill.clone(),
        };
        // ponytail: label width estimated from its length at the phone font size, no text
        // metrics offline. Centred above its point, kept inside the chart.
        let label_w = 8.0 * skill.chars().count() as f64;
        let x0 = (px - label_w / 2.0).clamp(0.0, (width - label_w).max(0.0));
        let label = (x0, py - 22.0, x0 + label_w, py - 6.0);
        let overlaps = |(a0, b0, a1, b1): (f64, f64, f64, f64)| {
            a0 < label.2 && label.0 < a1 && b0 < label.3 && label.1 < b1
        };
        if placed.len() < 8 && !placed.iter().chain(&points).copied().any(overlaps) {
            placed.push(label);
            let _ = write!(
                svg,
                "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">{}</text>",
                coord(x0 + label_w / 2.0),
                coord(py - 9.0),
                escape(&skill)
            );
        }
    }
    svg.push_str("</svg>\n");
    svg
}

const STYLE: &str = "
:root { color-scheme: light dark; --bg: #fff; --fg: #1a1a1a; --muted: #666; --line: #ddd;
  --s1: #2a78d6; --s2: #eb6834; --s3: #1baf7a; --s4: #eda100; --s5: #8b5cd6; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #161616; --fg: #e8e8e8; --muted: #9a9a9a; --line: #333;
    --s1: #3987e5; --s2: #d95926; --s3: #199e70; --s4: #c98500; --s5: #9d72e8; }
}
body { margin: 0 auto; max-width: 72rem; padding: 1rem; background: var(--bg); color: var(--fg);
  font: 15px/1.45 system-ui, sans-serif; }
p { color: var(--muted); }
.scroll { overflow-x: auto; }
table { border-collapse: collapse; font-variant-numeric: tabular-nums; }
th, td { padding: .35rem .6rem; border-bottom: 1px solid var(--line); text-align: right;
  white-space: nowrap; }
th[scope=row], thead th:first-child { text-align: left; }
fieldset { border: 0; padding: 0; margin: 0 0 1rem; }
legend { float: left; margin-right: .6rem; }
label { margin-right: .6rem; white-space: nowrap; }
summary { cursor: pointer; padding: .3rem 0; }
details table { margin-bottom: .8rem; }
svg { display: block; width: 100%; max-width: 32rem; height: auto; margin: 0 0 1.2rem;
  font: 12px system-ui, sans-serif; }
@media (max-width: 30rem) { svg { font-size: 14px; } }
svg text { fill: var(--muted); }
svg .grid { stroke: var(--line); }
polyline { fill: none; stroke-width: 2; }
svg line[class^=s] { stroke-width: 2; }
circle { stroke: var(--bg); stroke-width: 2; }
.s1 { stroke: var(--s1); } .s2 { stroke: var(--s2); }
.s3 { stroke: var(--s3); } .s4 { stroke: var(--s4); } .s5 { stroke: var(--s5); }
circle.s1, .dot { fill: var(--s1); } circle.s2 { fill: var(--s2); }
circle.s3 { fill: var(--s3); } circle.s4 { fill: var(--s4); } circle.s5 { fill: var(--s5); }
#view-slash, #view-model { display: none; }
body:has(#kind-slash:checked) #view-slash, body:has(#kind-model:checked) #view-model {
  display: block; }
body:has(#kind-slash:checked, #kind-model:checked) #view-all { display: none; }
";

/// The start-kind switch: id, label and the start kind it keeps, `all` first and checked.
/// STYLE names the ids; a new setting needs its rules there.
const KINDS: [(&str, &str, Option<&str>); 3] = [
    ("all", "all", None),
    ("slash", "/skill", Some("slash")),
    ("model", "model", Some("model")),
];

/// The switch and one view per setting. CSS shows the checked one, so the page needs no script.
fn views(calls: &[ReportCall], dates: &Dates) -> String {
    let mut page = String::from("<fieldset><legend>Calls started by</legend>");
    for (i, (id, label, _)) in KINDS.iter().enumerate() {
        let checked = if i == 0 { " checked" } else { "" };
        let _ = write!(
            page,
            "<label><input type=\"radio\" name=\"kind\" id=\"kind-{id}\"{checked}> {label}</label>"
        );
    }
    page.push_str("</fieldset>\n");
    for (id, _, kind) in KINDS {
        let skills = by_skill(calls, kind);
        let _ = write!(
            page,
            "<section id=\"view-{id}\">\n{}\n<h2>Calls against correction rate</h2>\n{}\
             <h2>Per skill</h2>\n{}</section>\n",
            overview(&skills),
            scatter(&skills),
            per_skill(&skills, dates)
        );
    }
    page
}

/// The whole page.
pub fn html(calls: &[ReportCall], dates: &Dates) -> String {
    let body = if calls.is_empty() {
        "<p>No skill calls yet.</p>".to_owned()
    } else {
        format!(
            "<p>A rate is the share of skill calls with at least one such event, with its 95 % \
             Wilson interval. A bare question is a choice question asked with no visible text since \
             the last tool call; its rate counts every call, also those that asked nothing. \
             Recommended is recommended answers over questions. A skill marked \
             nested never had a call of its own; its row counts the windows of other skills' \
             calls that loaded it. Those calls still count only in their own skill's row. Calls \
             whose parser did not measure a value stay out of its share. The switch limits every \
             number to calls started with /skill or by the model; a nested row follows the call \
             whose window loaded it. A call whose start kind was not recorded counts only under \
             all. Per skill, versions show its calls per skill version, commits by date, then \
             uncommitted edits, then calls without a version, and the trend shows each rate per \
             ISO week of the call's start. The scatter leaves out nested rows and skills whose \
             calls never measured corrections.</p>\n{}",
            views(calls, dates)
        )
    };
    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <title>Skill metrics</title>\n<style>{STYLE}</style>\n</head>\n<body>\n\
         <h1>Skill metrics</h1>\n{body}\n</body>\n</html>\n"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn percent((low, high): (f64, f64)) -> (i64, i64) {
        ((low * 100.0).round() as i64, (high * 100.0).round() as i64)
    }

    #[test]
    fn wilson_interval_at_zero_at_n_and_small_n() {
        assert_eq!(percent(wilson(0, 10)), (0, 28));
        assert_eq!(percent(wilson(10, 10)), (72, 100));
        assert_eq!(percent(wilson(1, 3)), (6, 79));
        assert_eq!(percent(wilson(1, 1)), (21, 100));
        assert_eq!(rate(1, 3), "33 % (6–79 %)");
        assert_eq!(rate(0, 0), "–");
    }

    #[test]
    fn version_columns_go_by_commit_date_then_unknown_uncommitted_none() {
        let dates: Dates = [
            ("b".to_owned(), (100, "1970-01-01".to_owned())),
            ("a".to_owned(), (200, "1970-01-01".to_owned())),
        ]
        .into();
        let mut columns = [Some("uncommitted"), None, Some("a"), Some("z"), Some("b")];
        columns.sort_by_key(|&version| column_order(version, &dates));
        assert_eq!(
            columns,
            [Some("b"), Some("a"), Some("z"), Some("uncommitted"), None]
        );
    }

    #[test]
    fn end_labels_spread_apart_and_stay_above_the_floor() {
        assert_eq!(spread(&[50.0, 50.0, 50.0], 10.0, 100.0), [50.0, 60.0, 70.0]);
        assert_eq!(
            spread(&[90.0, 90.0, 90.0, 90.0], 10.0, 100.0),
            [70.0, 80.0, 90.0, 100.0]
        );
        assert_eq!(spread(&[10.0, 80.0], 10.0, 100.0), [10.0, 80.0]);
    }

    #[test]
    fn median_of_odd_even_and_none() {
        assert_eq!(median(vec![3, 1, 2]), Some(2));
        assert_eq!(median(vec![4, 1, 3, 2]), Some(2));
        assert_eq!(median(vec![]), None);
    }

    #[test]
    fn tokens_switch_units_after_rounding() {
        assert_eq!(tokens(9_999), "9999");
        assert_eq!(tokens(247_400), "247k");
        assert_eq!(tokens(999_600), "1.0M");
        assert_eq!(tokens(5_240_000), "5.2M");
    }
}
