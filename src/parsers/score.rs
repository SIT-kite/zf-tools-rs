use crate::parsers::Semester;
use crate::Result;
use serde_json::Value;

#[derive(Clone)]
pub struct Score {
    /// 成绩
    score: f32,
    /// 课程
    course: String,
    /// 课程代码
    course_id: String,
    /// 班级
    class_id: String,
    /// 学年
    school_year: String,
    /// 学期
    semester: Semester,
    /// 学分
    credit: f32,
}

pub fn parse_score_list_page(page: &str) -> Result<Vec<Score>> {
    let json_page: Value = serde_json::from_str(page)?;

    let get_str = |x: Option<&Value>| -> String { x.map(ToString::to_string).unwrap_or_default() };
    let get_f32 = |x: Option<&Value>| -> f32 { get_str(x).parse().unwrap() };

    let result = json_page["items"].as_array().map(|course_list| {
        course_list
            .into_iter()
            .map(|course| Score {
                score: get_f32(course.get("cj")),
                course: get_str(course.get("kcmc")),
                course_id: get_str(course.get("kch")),
                class_id: get_str(course.get("jxb_id")),
                school_year: get_str(course.get("xnmmc")),
                semester: Semester::from_raw(get_str(course.get("xqm")).as_str()).unwrap(),
                credit: get_f32(course.get("xf")),
            })
            .collect()
    });
    match result {
        Some(v) => Ok(v),
        None => Ok(vec![]),
    }
}

pub fn calculate_gpa(scores: Vec<Score>) -> f32 {
    let mut total_credits = 0.0;
    let mut t = 0.0;
    for s in scores {
        t += s.credit * s.score;
        total_credits += s.credit;
    }
    let result = (t / total_credits / 10.0) - 5.0;
    return result;
}
