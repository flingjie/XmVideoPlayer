use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub fn merge_ts(temp_dir: &str, out_path: &str) {
    // 读取目录下所有文件路径
    // 可能包含其他非ts文件 如.DS_Store
    // TODO:过滤掉无关的文件
    let mut paths = std::fs::read_dir(temp_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    // 非数字顺序
    paths.sort();

    // let out_path = PathBuf::from(temp_dir);
    // let mut out_path = PathBuf::from(out_path.parent().unwrap());
    // out_path.push("output.ts");

    let mut f = File::create(&out_path).unwrap();
    for id in 0..paths.len() {
        let mut ts_path = PathBuf::from(temp_dir);
        ts_path.push(format!("{}.ts", id));
        read_file_buffer(&ts_path, &mut f);
    }
}

// 带缓冲区的读写
fn read_file_buffer(filepath: &PathBuf, out_file: &mut File) {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    if let Ok(mut f) = File::open(&filepath) {
        loop {
            let read_count = f.read(&mut buffer).unwrap();
            // println!("read_size: {}", read_count);
            // let write_count =
            out_file.write(&mut buffer[0..read_count]).unwrap();
            // println!("write_size: {}", write_count);
            if read_count != BUFFER_LEN {
                std::fs::remove_file(&filepath).unwrap();
                break;
            }
        }
    }
}
