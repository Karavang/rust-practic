// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use zip::{read::ZipArchive, write::FileOptions, CompressionMethod, ZipWriter};

// Проверяет наличие файла archive.zip в корневой директории и создает его, если отсутствует
fn check_archive_existence() -> io::Result<()> {
    let archive_path = "archive.zip";
    if !Path::new(archive_path).exists() {
        let file = File::create(archive_path)?;
        let mut zip = ZipWriter::new(file);
        zip.finish()?;
        println!("Создан новый архив: {}", archive_path);
    }
    Ok(())
}

#[tauri::command]
fn add_file_to_zip(file_name: &str) -> Result<(), tauri::Error> {
    let archive_path = "archive.zip";
    let file = File::open(archive_path)?;
    let mut zip = ZipWriter::new(file);

    let options = FileOptions::default().compression_method(CompressionMethod::Stored);
    let mut file_to_add = File::open(file_name);
    zip.start_file(file_name, options);
    io::copy(&mut file_to_add, &mut zip);
    println!("Файл '{}' добавлен в архив.", file_name);
    zip.finish();
    Ok(())
}

#[tauri::command]
fn remove_file_from_zip(file_name: &str) -> io::Result<()> {
    let archive_path = "archive.zip";
    let temp_archive_path = "temp.zip";

    let file = File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;
    let mut updated_zip = ZipWriter::new(File::create(temp_archive_path)?);

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let entry_name = entry.name().to_string();

        if entry_name != file_name {
            let mut buffer = Vec::new();
            entry.read_to_end(&mut buffer)?;

            let options = FileOptions::default()
                .compression_method(entry.compression())
                .unix_permissions(entry.unix_mode().unwrap_or(0o755));

            updated_zip.start_file(&entry_name, options)?;
            updated_zip.write_all(&buffer)?;
        }
    }

    updated_zip.finish()?;
    fs::remove_file(archive_path)?;
    fs::rename(temp_archive_path, archive_path)?;
    println!("Файл '{}' удален из архива.", file_name);
    Ok(())
}

#[tauri::command]
fn list_files_in_zip() -> io::Result<Vec<String>> {
    let archive_path = "archive.zip";
    let file = File::open(archive_path)?;
    let mut zip = ZipArchive::new(file)?;
    let mut files = Vec::new();

    for i in 0..zip.len() {
        let entry = zip.by_index(i)?;
        files.push(entry.name().to_string());
    }
    Ok(files)
}

fn main() -> io::Result<()> {
    check_archive_existence()?;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            add_file_to_zip,
            list_files_in_zip,
            remove_file_from_zip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
