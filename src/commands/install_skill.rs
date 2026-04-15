use anyhow::{Context, Result};
use std::fs;

const SKILL_CONTENT: &str = include_str!("../../.agents/skills/rustasana/SKILL.md");

pub fn run(force: bool) -> Result<()> {
    let home = dirs::home_dir().context("Unable to find home directory")?;

    // Install to multiple locations for compatibility
    let locations = vec![
        (home.join(".agents/skills/rustasana"), "Generic AI agents"),
        (home.join(".config/opencode/skills/rustasana"), "OpenCode"),
        (home.join(".claude/skills/rustasana"), "Claude Desktop"),
        (home.join(".cursor/skills/rustasana"), "Cursor"),
    ];

    println!("🚀 Installing Rustasana AI skill globally...\n");

    let mut installed_count = 0;
    let mut skipped_count = 0;

    for (skill_dir, tool_name) in locations {
        let skill_file = skill_dir.join("SKILL.md");

        // Check if already exists
        if skill_file.exists() && !force {
            println!("⏭️  Skipped {} - already exists", tool_name);
            println!("   Location: {}", skill_file.display());
            println!("   Use --force to overwrite\n");
            skipped_count += 1;
            continue;
        }

        // Create directory
        fs::create_dir_all(&skill_dir)
            .with_context(|| format!("Failed to create directory: {}", skill_dir.display()))?;

        // Write skill file
        fs::write(&skill_file, SKILL_CONTENT)
            .with_context(|| format!("Failed to write skill file: {}", skill_file.display()))?;

        println!("✅ Installed for {}", tool_name);
        println!("   Location: {}", skill_file.display());
        println!();
        installed_count += 1;
    }

    // Summary
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊 Summary:");
    println!("   ✅ Installed: {}", installed_count);
    if skipped_count > 0 {
        println!("   ⏭️  Skipped:   {}", skipped_count);
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    if installed_count > 0 {
        println!("🎉 Success! The Rustasana skill is now installed globally.");
        println!();
        println!("📚 Usage:");
        println!("   Start your AI tool (OpenCode, Claude, Cursor, etc.) and say:");
        println!("   \"Load the rustasana skill\" or \"Show my Asana tasks using rustasana\"");
        println!();
        println!("🔍 Verify:");
        println!("   opencode        # Then ask: \"What skills do you have?\"");
        println!("   cursor          # Skills auto-detected");
        println!();
        println!("📖 Learn more:");
        println!("   rustasana --help");
        println!("   cat ~/.agents/skills/rustasana/SKILL.md");
    } else {
        println!("ℹ️  No new installations. All locations already have the skill.");
        println!("   Use 'rustasana install-skill --force' to reinstall.");
    }

    Ok(())
}
