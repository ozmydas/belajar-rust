fn train_ninja_long_way(ninja: Option<&str>, chakra: Result<u32, &str>) -> Result<String, String> {
    let ninja_name = match ninja {
        Some(name) => name,
        None => return Err("No ninja found".to_string()),  // ❌ Early return
    };

    let chakra_level = match chakra {
        Ok(level) => level,
        Err(err) => return Err(err.to_string()),  // ❌ Early return
    };

    Ok(format!("{} is ready to train with {} chakra!", ninja_name, chakra_level))
}


fn train_ninja_short_way(ninja: Option<&str>, chakra: Result<u32, &str>) -> Result<String, String> {
    let ninja_name = ninja.ok_or("No ninja found")?;  // 🪄 Auto return error if None
    let chakra_level = chakra?;  // 🪄 Auto return error if Err

    Ok(format!("{} is ready to train with {} chakra!", ninja_name, chakra_level))
}

/*********************************/

fn do_some_math() -> Result<f64, &'static str> {
    let r1 = divider(10.0, 5.0)?;
    println!("result: {r1:.2}");
    
    let r2 = divider(10.0, 2.5)?;
    println!("result: {r2:.2}");
    
    let r3 = divider(10.0, 2.0)?;
    println!("result: {r3:.2}");

    // if up there have some errors, then error will returned and function stopped
    // if no errors, below is returned
    return Ok(r1+r2+r3)
}

fn divider(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("division by zero error");
    }

    let result = a / b;
    return Ok(result);
}

/*********************************/

fn main() {
    let ninja1 = train_ninja_long_way(Some("Naruto"), Ok(100));
    let ninja2 = train_ninja_short_way(Some("Sasuke"), Ok(100));
    let ninja3 = train_ninja_short_way(None, Ok(80));   // ❌ No ninja found!
    let ninja4 = train_ninja_short_way(Some("Sasuke"), Err("Chakra too low!")); // ❌ Error

    // Print results
    match ninja1 {
        Ok(msg) => println!("✅ {msg}"),
        Err(err) => println!("❌ Error: {err}"),
    }

    match ninja2 {
        Ok(msg) => println!("✅ {msg}"),
        Err(err) => println!("❌ Error: {err}"),
    }

    match ninja3 {
        Ok(msg) => println!("✅ {msg}"),
        Err(err) => println!("❌ Error: {err}"),
    }

    match ninja4 {
        Ok(msg) => println!("✅ {msg}"),
        Err(err) => println!("❌ Error: {err}"),
    }

    /***********/

    println!("======================================");

    match do_some_math() {
        Ok(ok) => println!("OK total : {:?}", ok),
        Err(err) => println!("Error cuy : {:?}", err),
    }
}