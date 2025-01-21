use lazy_regex::regex;
use std::collections::HashSet;

//matches arbitrary values like '[prefix:value]'
pub static  ARBITRARY_VALUE_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"^\[(?:([a-z-]+):)?(.+)\]$");

// Matches fractions like `1/2`, `3/4`
pub static FRACTION_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"^\d+/\d+$");

// Predefined string lengths
pub fn string_lengths() -> HashSet<&'static str> {
  let mut string_set = HashSet::new();
  string_set.insert("px");
  string_set.insert("full");
  string_set.insert("screen");
  string_set
}

pub fn size_labels() -> HashSet<&'static str> {
  let mut size_set = HashSet::new();
  size_set.insert("length");
  size_set.insert("size");
  size_set.insert("percentage");
}

pub fn image_labels() -> HashSet<&'static str> {
  let mut image_set = HashSet::new();
  image_set.insert("image");
  image_set.insert("url");
}

// Matches t-shirt size units like `2xl`, `md`, `lg`
pub static TSHIRT_UNIT_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"^(\d+(\.\d+)?)?(xs|sm|md|lg|xl)$");

// Matches valid length units like `px`, `rem`, `vh`, etc.
pub static LENGTH_UNIT_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"\d+(%|px|r?em|[sdl]?v([hwib]|min|max)|pt|pc|in|cm|mm|cap|ch|ex|r?lh|cq(w|h|i|b|min|max))|\b(calc|min|max|clamp)\(.+\)|^0$");

// Matches valid CSS color functions like `rgb(...)`, `hsl(...)`, `lab(...)`
pub static COLOR_FUNCTION_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"^(rgba?|hsla?|hwb|(ok)?(lab|lch))\(.+\)$");

// Matches box shadows like `inset_4px_4px`
pub static SHADOW_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"^(inset_)?-?((\d+)?\.?(\d+)[a-z]+|0)_-?((\d+)?\.?(\d+)[a-z]+|0)");

// Matches image-related functions like `url(...)`, `linear-gradient(...)`
pub static IMAGE_REGEX: lazy_regex::Lazy<regex::Regex> = regex!(r"^(url|image|image-set|cross-fade|element|(repeating-)?(linear|radial|conic)-gradient)\(.+\)$");

pub fn is_length(value: &str) -> bool {
  is_number(value) || string_lengths().contains(value) || FRACTION_REGEX.is_match(value)
}

pub fn is_number(value: &str) -> bool {
  value.parse::<f64>().is_ok()
}

pub fn is_integer(value: &str) -> bool {
  value.parse::<f64>().map_or(false, |num| num.fract() == 0.0)
}

pub fn is_percent(value: &str) -> bool {
  if let Some(stripped) = value.strip_suffix('%') {
    is_number(stripped)
  } else {
    false
  }
}

pub fn is_arbitrary_value(value: &str) -> bool {
  ARBITRARY_VALUE_REGEX.is_match(value)
}

pub fn is_tshirt_size(value: &str) -> bool {
  TSHIRT_UNIT_REGEX.is_match(value)
}

pub fn is_arbitrary_number(value: &str) -> bool {
  get_is_arbitrary_value(value, "number", is_number(value))
}

pub fn is_arbitrary_length(value: &str) -> bool {
  get_is_arbitrary_value(value, "length", is_length_only)
}

pub fn is_arbitrary_size(value: &str) -> bool {
  get_is_arbitrary_value(value, "size-labels", is_never())
}

pub fn is_arbitrary_position(value: &str) -> bool {
  get_is_arbitrary_value(value, "position", test_value)
}

pub fn is_arbitrary_image(value: &str) -> bool {
  get_is_arbitrary_value(value, "image-labels", is_image)
}

pub fn is_arbitrary_shadow(value: &str) -> bool {
  get_is_arbitrary_value(value, " ", is_shadow)
}

pub fn get_is_arbitrary_value(
  value: &str,
  label: &str,
  test_value: F,
) -> bool 
where F: Fn(&str) -> bool,
{
  if let Some(captures) = ARBITRARY_VALUE_REGEX.captures(value) {
    if let Some(label_match) = captures.get(1) {
      return label == label_match.as_str();
    }
    //no label is found use provided test function
    return test_value(captures.get(2).unwrap().as_str());
  }
  false
}

pub fn is_length_only(value: &str) -> bool {
  // `colorFunctionRegex` check is necessary because color functions can have percentages in them which which would be incorrectly classified as lengths.
  // For example, `hsl(0 0% 0%)` would be classified as a length without this check.
  LENGTH_UNIT_REGEX.is_match(value) && !COLOR_FUNCTION_REGEX.is_match(value)
}

pub fn is_never() -> bool {
  false
}

pub fn is_shadow(value: &str) -> bool {
  SHADOW_REGEX.is_match(value)
}

pub fn is_image(value: &str) -> bool {
  IMAGE_REGEX.is_match(value)
}