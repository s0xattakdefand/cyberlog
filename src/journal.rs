use chrono::Utc;
use dialoguer::Input;
use rusqlite::{params};
use prettytable::{Table, row};

use crate::db::get_connection;

pub fn handle_new(role: String) -> Result<(), Box<dyn std::error::Error>> {
    let date = Utc::now().format("%Y-%m-%d").to_string();

    let actions: String = Input::new()
        .with_prompt("What did you do today? (steps/commands)")
        .interact_text()?;

    let results: String = Input::new()
        .with_prompt("What failed or succeeded?")
        .interact_text()?;

    let vulnerabilities: String = Input::new()
        .with_prompt("Where was the vulnerability or hardening?")
        .interact_text()?;

    let discoveries: String = Input::new()
        .with_prompt("Any new tools or directories?")
        .interact_text()?;

    let conn = get_connection()?;
    conn.execute(
        "INSERT INTO logs (date, role, actions, results, vulnerabilities, discoveries)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![date, role, actions, results, vulnerabilities, discoveries],
    )?;

    println!("✅ Journal entry saved for {}", date);
    Ok(())
}

pub fn handle_list() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, date, role, actions, results, vulnerabilities, discoveries FROM logs ORDER BY id DESC"
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    let mut table = Table::new();
    table.add_row(row!["ID", "Date", "Role", "Actions", "Results", "Vuln", "Discoveries"]);

    for row in rows {
        let (id, date, role, actions, results, vulns, discoveries) = row?;
        table.add_row(row![
            id,
            date,
            role,
            actions.chars().take(30).collect::<String>(),
            results.chars().take(30).collect::<String>(),
            vulns.chars().take(30).collect::<String>(),
            discoveries.chars().take(30).collect::<String>(),
        ]);
    }

    table.printstd();
    Ok(())
}

pub fn handle_view(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, date, role, actions, results, vulnerabilities, discoveries FROM logs WHERE id = ?1"
    )?;

    let mut rows = stmt.query_map([id], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    if let Some(row) = rows.next() {
        let (id, date, role, actions, results, vulnerabilities, discoveries) = row?;
        println!("🔍 Journal Entry ID: {id}");
        println!("📅 Date:              {date}");
        println!("🧑 Role:              {role}");
        println!("🛠️  Actions:\n{actions}");
        println!("✅❌ Results:\n{results}");
        println!("🔓🔒 Vulnerabilities / Hardening:\n{vulnerabilities}");
        println!("🗂 Discoveries:\n{discoveries}");
    } else {
        println!("⚠️ No journal entry found with ID {id}");
    }

    Ok(())
}

pub fn handle_delete(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;
    let rows_affected = conn.execute("DELETE FROM logs WHERE id = ?1", [id])?;
    if rows_affected > 0 {
        println!("🗑️  Entry with ID {id} deleted.");
    } else {
        println!("⚠️  No entry found with ID {id}");
    }
    Ok(())
}

pub fn handle_edit(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT actions, results, vulnerabilities, discoveries FROM logs WHERE id = ?1"
    )?;
    let mut rows = stmt.query([id])?;

    if let Some(row) = rows.next()? {
        let current_actions: String = row.get(0)?;
        let current_results: String = row.get(1)?;
        let current_vulns: String = row.get(2)?;
        let current_discoveries: String = row.get(3)?;

        let actions: String = Input::new()
            .with_prompt("Update actions:")
            .default(current_actions)
            .interact_text()?;

        let results: String = Input::new()
            .with_prompt("Update results:")
            .default(current_results)
            .interact_text()?;

        let vulnerabilities: String = Input::new()
            .with_prompt("Update vulnerabilities:")
            .default(current_vulns)
            .interact_text()?;

        let discoveries: String = Input::new()
            .with_prompt("Update discoveries:")
            .default(current_discoveries)
            .interact_text()?;

        conn.execute(
            "UPDATE logs SET actions = ?1, results = ?2, vulnerabilities = ?3, discoveries = ?4 WHERE id = ?5",
            params![actions, results, vulnerabilities, discoveries, id],
        )?;

        println!("✅ Entry ID {id} updated.");
    } else {
        println!("⚠️  No entry found with ID {id}");
    }

    Ok(())
}

pub fn handle_search(field: String, keyword: String) -> Result<(), Box<dyn std::error::Error>> {
    let allowed_fields = ["actions", "results", "vulnerabilities", "discoveries"];
    if !allowed_fields.contains(&field.as_str()) {
        println!("⚠️ Invalid field. Use one of: actions, results, vulnerabilities, discoveries");
        return Ok(());
    }

    let conn = get_connection()?;
    let sql = format!(
        "SELECT id, date, role, actions, results, vulnerabilities, discoveries FROM logs WHERE {} LIKE ?1 ORDER BY id DESC",
        field
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([format!("%{}%", keyword)], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
            row.get::<_, String>(5)?,
            row.get::<_, String>(6)?,
        ))
    })?;

    let mut table = Table::new();
    table.add_row(row!["ID", "Date", "Role", "Actions", "Results", "Vuln", "Discoveries"]);

    for row in rows {
        let (id, date, role, actions, results, vulns, discoveries) = row?;
        table.add_row(row![
            id,
            date,
            role,
            actions.chars().take(30).collect::<String>(),
            results.chars().take(30).collect::<String>(),
            vulns.chars().take(30).collect::<String>(),
            discoveries.chars().take(30).collect::<String>(),
        ]);
    }

    table.printstd();
    Ok(())
}
