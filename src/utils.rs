
pub fn cmp_vec_unordered<T>(v1: &Vec<T>, mut v2: Vec<T>) -> bool
  where T: PartialOrd + std::fmt::Debug {
  for x in v1 {
    if v2.contains(&x) {
      let index = v2.iter().position(|x1| x1 == x);

      if index.is_none() {
        return false
      }

      v2.remove(index.unwrap());
      continue;
    }

    return false
  }

  if v2.len() > 0 {
    return false
  }

  return true
}
