use rusqlite::{Connection};
use serde_json::{Value,json};

use std::ptr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::ffi::OsStringExt;

use winapi::um::winuser::{GetDesktopWindow, MessageBoxW,MB_ICONQUESTION, MB_YESNO, IDYES, IDNO};
use winapi::um::commdlg::{GetOpenFileNameW,OFN_ALLOWMULTISELECT, OFN_EXPLORER, OFN_FILEMUSTEXIST, OFN_PATHMUSTEXIST, OPENFILENAMEW};
use winapi::shared::{windef::HWND,minwindef::LPVOID};


#[derive(Debug)]
struct Res {
    json: String,
}


 
fn changeRate(dbpath:String,rate:u32){
    let mut conn = Connection::open(dbpath).unwrap();
     

    let mut stmt = conn.prepare(
        "SELECT value FROM LocalStorage WHERE key = 'GameQualitySetting'",
        ).unwrap();
    let mut res = stmt.query_map([], |row| {
        Ok(Res{
            json: row.get(0).unwrap(),
        })
    }).unwrap();
     
    let r = res.next().unwrap().unwrap().json;
    println!("read result:{}",r);
    
    let mut v:Value = serde_json::from_str(&r).unwrap();
    v["KeyCustomFrameRate"] = json!(rate) ;
    println!("after write:{}",v);
    let update_value = v.to_string();
    
    let _ = conn.execute("update LocalStorage set  value =  (?1) WHERE key = 'GameQualitySetting'", &[&update_value]);
    
}


fn openFile()->String{
    let mut path = "".to_string() ;
    unsafe{
        // 定义文件名缓冲区
        let mut file_name: [u16; winapi::shared::minwindef::MAX_PATH] = [0; winapi::shared::minwindef::MAX_PATH];

        // 创建 OPENFILENAME 结构体
        let mut ofn: OPENFILENAMEW = std::mem::zeroed();
        ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
        ofn.hwndOwner = GetDesktopWindow();
        ofn.lpstrFile = file_name.as_mut_ptr();
        ofn.nMaxFile = winapi::shared::minwindef::MAX_PATH as u32;
        ofn.Flags = OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_ALLOWMULTISELECT;

        // 显示文件对话框
        if GetOpenFileNameW(&mut ofn) != 0 {
            // 将选择的文件路径转换为 Rust 字符串
            let path_os_string: OsString = OsString::from_wide(&file_name);
            path = path_os_string.into_string().unwrap();
            
        }

    }
    path
}



fn confirm(title:&str,message:&str)->bool{
    let mut res = false;
    unsafe {
        // 设置确认框的标题和消息内容
        // let title = "确认";
        // let message = "确认?";
        // 将标题和消息内容转换为宽字符
        let title_wide: Vec<u16> = OsString::from(title).encode_wide().chain(Some(0)).collect();
        let message_wide: Vec<u16> = OsString::from(message).encode_wide().chain(Some(0)).collect();

        // 调用 MessageBoxW 函数显示确认框
        let result = MessageBoxW(
            ptr::null_mut(),
            message_wide.as_ptr(),
            title_wide.as_ptr(),
            MB_ICONQUESTION | MB_YESNO,
        );

        // 处理用户的选择
        if result == IDYES {
            res = true;
        } else if result == IDNO {
            res = false;
        }
    }
    res
}

fn main() {
    let mut path: String = "".to_string();
    let title = "请打开游戏目录";
    let message = "目录错误，是否继续选择文件?";
    for n in 0..10{
        path = openFile();
        println!("打开目录为{}",path);
        if !path.contains("Wuthering Waves"){
            if !confirm(title, message){
                return ;
            }
        }else{
            break;
        }
        if n==9{
            println!("死猪都选对了,你咋就嫩能呢你");
        }
    }
    println!("目录在:{}",path.trim_matches(char::from(0)));
    let _ = changeRate(path.trim_matches(char::from(0)).to_string(),120);
}
