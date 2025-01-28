#![allow(unused)]
use std::{collections::HashMap, hash::Hash, sync::Arc};

use crate::types::{AnyThemeGroupIds, ClassGroup, ThemeGetter, ThemeObject};

pub fn from_theme<ThemeGroupIds: Clone + Eq + Hash + std::fmt::Debug + Sync + Send + 'static>(key: ThemeGroupIds) -> ThemeGetter<ThemeGroupIds>{
  let key_clone = key.clone();
  ThemeGetter {
    function: Arc::new(move |theme: &ThemeObject<ThemeGroupIds>| {
      theme.get(&key_clone)
        .cloned()
        .unwrap_or_else(Vec::new)
    }),
    is_theme_getter: true,   
  }
 
}