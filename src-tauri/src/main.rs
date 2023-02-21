#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use chrono::{Datelike, Days, NaiveDate};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::Mutex;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Task {
    id: i32,
    detail: String,
    year: i32,
    month: i32,
    day: i32,
    tag: String,
}

lazy_static! {
    static ref DATA_FILE: Mutex<File> = Mutex::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("./data.txt")
            .unwrap()
    );
    static ref TASK_ID: Mutex<i32> = Mutex::new(0);
    static ref TASK_LIST: Mutex<Vec<Task>> = Mutex::new(vec![]);
}

fn update_file() {
    let mut data_file = DATA_FILE.lock().unwrap();
    data_file.set_len(0).unwrap();
    data_file.seek(SeekFrom::Start(0)).unwrap();

    let data_source = (
        *TASK_ID.lock().unwrap(),
        (*TASK_LIST.lock().unwrap()).clone(),
    );
    let data = serde_json::to_string(&data_source).unwrap();
    data_file.write_all(data.as_bytes()).unwrap();
}

#[tauri::command]
fn add_task(detail: String, year: i32, month: i32, day: i32, tag: String) {
    {
        let mut task_id = TASK_ID.lock().unwrap();
        let mut task_list = TASK_LIST.lock().unwrap();
        *task_id += 1;
        task_list.push(Task {
            id: *task_id,
            detail,
            year,
            month,
            day,
            tag,
        });
    }
    update_file();
}

#[tauri::command]
fn delete_task(id: i32) {
    // println!("delete_task {}", id);
    {
        let mut task_list = TASK_LIST.lock().unwrap();
        *task_list = task_list
            .iter()
            .filter(|task| id != task.id)
            .map(|task| task.clone())
            .collect::<Vec<_>>();
    }
    // println!("deleted");
    update_file();
    // println!("updated");
}

fn task_cmp(a: &Task, b: &Task) -> Ordering {
    if a.year != b.year {
        return a.year.cmp(&b.year);
    }
    if a.month != b.month {
        return a.month.cmp(&b.month);
    }
    a.day.cmp(&b.day)
}

#[tauri::command]
fn get_tags() -> Vec<String> {
    let task_list = TASK_LIST.lock().unwrap();
    let mut res: HashSet<String> = HashSet::new();
    for task in task_list.iter() {
        res.insert(task.tag.clone());
    }
    let mut res: Vec<_> = res.into_iter().collect();
    res.sort();
    res
}

#[tauri::command]
fn get_tasks() -> Vec<Task> {
    let task_list = TASK_LIST.lock().unwrap();
    let mut res = task_list.clone();
    res.sort_by(|a, b| task_cmp(a, b));
    res
}

#[tauri::command]
fn get_tasks_by_tag(tag: String) -> Vec<Task> {
    let mut res = TASK_LIST
        .lock()
        .unwrap()
        .iter()
        .filter(|task| task.tag == tag)
        .map(|task| task.clone())
        .collect::<Vec<_>>();
    res.sort_by(|a, b| task_cmp(a, b));
    res
}

#[tauri::command]
fn get_tasks_by_time(
    begin_year: i32,
    begin_month: i32,
    begin_day: i32,
    end_year: i32,
    end_month: i32,
    end_day: i32,
) -> Vec<Task> {
    // println!(
    //     "get_tasks_by_time {} {} {} {} {} {}",
    //     begin_year, begin_month, begin_day, end_year, end_month, end_day
    // );
    let begin_task = Task {
        id: 0,
        detail: String::new(),
        year: begin_year,
        month: begin_month,
        day: begin_day,
        tag: String::new(),
    };
    let end_task = Task {
        id: 0,
        detail: String::new(),
        year: end_year,
        month: end_month,
        day: end_day,
        tag: String::new(),
    };

    let mut res = TASK_LIST
        .lock()
        .unwrap()
        .iter()
        .filter(|task| {
            task_cmp(&task, &begin_task) != Ordering::Less
                && task_cmp(&task, &end_task) != Ordering::Greater
        })
        .map(|task| task.clone())
        .collect::<Vec<_>>();
    res.sort_by(|a, b| task_cmp(a, b));
    res
}

#[tauri::command]
fn get_tasks_by_days(
    year: i32,
    month: i32,
    day: i32,
    begin_offset: i32,
    end_offset: i32,
) -> Vec<Task> {
    // println!("get_tasks_by_days {} {} {} {} {}", year, month, day, begin_offset, end_offset);
    let original_date = NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap();
    let begin_date = original_date
        .checked_add_days(Days::new(begin_offset as u64))
        .unwrap();
    let end_date = original_date
        .checked_add_days(Days::new(end_offset as u64))
        .unwrap();
    get_tasks_by_time(
        begin_date.year(),
        begin_date.month() as i32,
        begin_date.day() as i32,
        end_date.year(),
        end_date.month() as i32,
        end_date.day() as i32,
    )
}

fn main() {
    let mut data_str = String::new();
    DATA_FILE
        .lock()
        .unwrap()
        .read_to_string(&mut data_str)
        .unwrap();
    (*TASK_ID.lock().unwrap(), *TASK_LIST.lock().unwrap()) =
        serde_json::from_str(&data_str).unwrap();
    // println!("{}", data_str);
    // println!("{:#?}", (*TASK_ID.lock().unwrap(), (*TASK_LIST.lock().unwrap()).clone()));

    // *TASK_ID.lock().unwrap() = 42;
    // TASK_LIST.lock().unwrap().push(Task {
    //     id: 42,
    //     detail: "Task Detail.".to_string(),
    //     year: 2077,
    //     month: 4,
    //     day: 1,
    //     tag: vec!["Tag_A".to_string(), "Tag_B".to_string()]
    // });
    // update_file();
    // return ();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_task,
            delete_task,
            get_tags,
            get_tasks,
            get_tasks_by_tag,
            get_tasks_by_time,
            get_tasks_by_days
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
