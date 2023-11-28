use chrono::NaiveDate;
use mysql::{Pool, PooledConn};

struct WorkPlace {
    id: Option<i32>,
    area: Option<u32>,
    address: Option<String>,
}

struct Lab {
    id: Option<i32>,
    name: Option<String>,
    introduction: Option<String>,
    work_place_id: Option<i32>,
}

struct Researcher {
    lab_id: Option<i32>,
    id: Option<i32>,
    name: Option<String>,
    gender: Option<String>,
    title: Option<String>,
    age: Option<i32>,
    research_direction: Option<String>,
}

struct Director {
    researcher_id: Option<i32>,
    start_date: Option<NaiveDate>,
    term: Option<i32>,
}

struct Secretary {
    lab_id: Option<i32>,
    id: Option<i32>,
    name: Option<String>,
    gender: Option<String>,
    age: Option<i32>,
    start_date: Option<NaiveDate>,
    duty: Option<String>,
}

struct ResearchProject {
    id: Option<i32>,
    leader_id: Option<i32>,
    name: Option<String>,
    content: Option<String>,
    funding: Option<f64>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
}

struct ProjectClient {
    id: Option<i32>,
    project_id: Option<i32>,
    name: Option<String>,
    address: Option<String>,
    leader_name: Option<String>,
    leader_telephone: Option<String>,
    leader_phone: Option<String>,
    leader_email: Option<String>,
    contact_name: Option<String>,
    contact_telephone: Option<String>,
    contact_phone: Option<String>,
    contact_email: Option<String>,
}

struct ProjectCollaborator {
    id: Option<i32>,
    project_id: Option<i32>,
    name: Option<String>,
    address: Option<String>,
    leader_name: Option<String>,
    leader_telephone: Option<String>,
    leader_phone: Option<String>,
    leader_email: Option<String>,
    contact_name: Option<String>,
    contact_telephone: Option<String>,
    contact_phone: Option<String>,
    contact_email: Option<String>,
}

struct QualityMonitor {
    id: Option<i32>,
    project_id: Option<i32>,
    name: Option<String>,
    address: Option<String>,
    leader_name: Option<String>,
    leader_telephone: Option<String>,
    leader_phone: Option<String>,
    leader_email: Option<String>,
    contact_name: Option<String>,
    contact_telephone: Option<String>,
    contact_phone: Option<String>,
    contact_email: Option<String>,
}

struct ProjectParticipant {
    project_id: Option<i32>,
    researcher_id: Option<i32>,
    join_date: Option<NaiveDate>,
    workload: Option<i32>,
    disposable_funds: Option<f64>,
}

struct ProjectSubtask {
    project_id: Option<i32>,
    leader_id: Option<i32>,
    subtask_num: Option<i32>,
    deadline: Option<NaiveDate>,
    disposable_funds: Option<f64>,
    technical_indicators: Option<String>,
}

struct ResearchOutcome {
    id: Option<i32>,
    project_id: Option<i32>,
    name: Option<String>,
    achieve_date: Option<NaiveDate>,
    rank_num: Option<i32>,
}

struct OutcomeContributor {
    id: Option<i32>,
    outcome_id: Option<i32>,
    researcher_id: Option<i32>,
}

struct Patent {
    id: Option<i32>,
    outcome_id: Option<i32>,
    paten_type: Option<String>,
}

struct Paper {
    id: Option<i32>,
    outcome_id: Option<i32>,
}

struct SoftwareCopyright {
    id: Option<i32>,
    outcome_id: Option<i32>,
}

pub struct DBHandler {
    conn: PooledConn,
}

impl DBHandler {
    pub fn new() -> DBHandler {
        let url = "mysql://jomo:12onetwo@192.168.168.132:3306/SRDB";
        let pool = Pool::new(url).unwrap();
        let conn = pool.get_conn().unwrap();
        return DBHandler { conn };
    }

    // todo
    pub fn create_table() {}
    pub fn insert_data() {}
    pub fn delete_table() {}
    pub fn delete_data() {}
    pub fn update() {}
    pub fn select() {}
    pub fn showtables() {}
}
