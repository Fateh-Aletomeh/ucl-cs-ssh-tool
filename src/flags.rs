use crate::Machine;
use std::cmp::max;


pub fn list_machines(lab_105: &[Machine], lab_121: &[Machine]) {
  println!("┌{:─<16}─┬─{:─<16}┐", "", "");
  println!("│ {:<15} │ {:<15} │", "1.05", "1.21");
  println!("├{:─<16}─┼─{:─<16}┤", "", "");

  let max_len = max(lab_105.len(), lab_121.len());
  for i in 0..max_len {
    let mach_105 = lab_105.get(i).map_or("", |m| &m.name);
    let mach_121 = lab_121.get(i).map_or("", |m| &m.name);

    println!("│ {:<15} │ {:<15} │", mach_105, mach_121);
  }

  println!("└{:─<16}─┴─{:─<16}┘", "", "");
}
