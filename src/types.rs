#![allow(unused)]
use strum::{Display, EnumIter, EnumString};
use std::{collections::HashMap, fmt::{Debug, Formatter}, hash::Hash, marker::PhantomData, sync::Arc};

//represents the static part of the Tailwind Merge Config
#[derive(Debug, Clone)]
pub struct ConfigStaticPart {
  /// Integer indicating the size of the LRU cache
  pub cache_size: usize,
  /// Optional prefix
  pub prefix: Option<String>, 
  // Custom separator for tailwind class modifiers
  pub separator: String, 
  // Experimental function for parsing class names
  pub experimental_parse_class_name: Option<fn(&ExperimentalParseClassName) -> ExperimentalParsedClassName>,
}

// Parameters for the experimental class name parsing function
#[derive(Debug, Clone)]
pub struct ExperimentalParseClassName {
  pub class_name: String, 
  pub parse_class_name: fn(&str) -> ExperimentalParseClassName,
}

// Result for experimental class name parsing
#[derive(Debug, Clone)]
pub struct ExperimentalParsedClassName {
  pub modifiers: Vec<String>,
  pub has_important_modifier: bool,
  pub base_class_name: String,
  pub maybe_postfix_modifier_position: Option<usize>,
}

// Represents the dynamic part
#[derive(Debug, Clone)]
pub struct ConfigGroupsPart<ClassGroupIds, ThemeGroupIds> 
where  
  ClassGroupIds: Eq + Hash + Clone + Debug,
  ThemeGroupIds: Eq + Hash + Clone + Debug,
{
  // Theme scales used in class groups
  pub theme: HashMap<ThemeGroupIds, ClassGroup<ThemeGroupIds>>,
  // Object mapping class group ids to their classes.
  pub class_groups: HashMap<ClassGroupIds, ClassGroup<ThemeGroupIds>>,
  // Conflicting class groups
  pub conflicting_class_groups: HashMap<ClassGroupIds, Vec<ClassGroupIds>>,
  // Conflicting class group modifiers 
  pub conflicting_class_group_modifiers: HashMap<ClassGroupIds, Vec<ClassGroupIds>>,
}

// Represents the entire tailwind merge configuration
#[derive(Debug, Clone)]
pub struct Config<ClassGroupIds, ThemeGroupIds>
where 
  ClassGroupIds: Eq + Hash + Clone + Debug,
  ThemeGroupIds: Eq + Hash + Clone + Debug,  
{
  pub static_part: ConfigStaticPart,
  pub groups_part: ConfigGroupsPart<ClassGroupIds, ThemeGroupIds>,
}

// Extension configuration for Tailwind Merge
#[derive(Debug, Clone)]
pub struct ConfigExtension<ClassGroupIds, ThemeGroupIds> 
where 
  ClassGroupIds: Eq + Hash + Clone + Debug,
  ThemeGroupIds: Eq + Hash + Clone + Debug, 
{
  pub override_part: Option<ConfigGroupsPart<ClassGroupIds, ThemeGroupIds>>,
  pub extend_part: Option<ConfigGroupsPart<ClassGroupIds, ThemeGroupIds>>,
}

// Theme object containing class groups
pub type ThemeObject<ThemeGroupIds> = HashMap<ThemeGroupIds, ClassGroup<ThemeGroupIds>>;

// class groups definition
pub type ClassGroup<ThemeGroupIds> = Vec<ClassDefinition<ThemeGroupIds>>;

// Enum defining the possible types of class definitiions
#[derive(Debug, Clone)]
pub enum ClassDefinition<ThemeGroupIds: Clone + Debug> {
  String(String), 
  ClassValidator(fn(&str)-> bool),
  ThemeGetter(ThemeGetter<ThemeGroupIds>), 
  ClassObject(HashMap<String, Vec<ClassDefinition<ThemeGroupIds>>>),
  _MARKER(PhantomData<ThemeGroupIds>),
}

// funtion type for retrieving theme-based class groups
#[derive(Clone)]
pub struct ThemeGetter<ThemeGroupIds: Debug + Clone> {
  pub function: Arc<dyn Fn(&ThemeObject<ThemeGroupIds>) -> ClassGroup<ThemeGroupIds> + Send + Sync>,
  pub is_theme_getter: bool,
} 

//manually implementing the Debug for ThemeGetter
impl<ThemeGroupIds: Debug + Clone> Debug for ThemeGetter<ThemeGroupIds> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ThemeGetter")
      .field("is_theme_getter", &self.is_theme_getter)
      .finish() //avoid trying to debug the fn
  }
}

// Disables the type inference 
pub struct NoInfer<T>(pub T);

pub type AnyClassGroupIds = String;

pub type AnyThemeGroupIds = String;

// pub struct Config<C, T> {
//   class_group_id: C,
//   theme_group_id: T,
// }

pub type AnyConfig = Config<AnyClassGroupIds, AnyThemeGroupIds>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, Display)]
#[strum(serialize_all = "camelCase")]
pub enum DefaultThemeGroupIds {
  Blur,
  BorderColor,
  BorderRadius,
  BorderSpacing,
  BorderWidth,
  Brightness,
  Colors,
  Contrast,
  Gap,
  GradientColorStopPositions,
  GradientColorStops,
  Grayscale,
  HueRotate,
  Inset,
  Invert,
  Margin,
  Opacity,
  Padding,
  Saturate,
  Scale,
  Sepia,
  Skew,
  Space,
  Spacing,
  Translate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, EnumIter, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum DefaultClassGroupIds {
  Accent,
  AlignContent,
  AlignItems,
  AlignSelf,
  Animate,
  Appearance,
  Aspect,
  AutoCols,
  AutoRows,
  BackdropBlur,
  BackdropBrightness,
  BackdropContrast,
  BackdropFilter,
  BackdropGrayScale,
  BackdropHueRotate,
  BackdropInvert,
  BackdropOpacity,
  BackdropSaturate,
  BackdropSepia,
  Basis,
  BgAttachment,
  BgBlend,
  BgClip,
  BgColor,
  BgImage,
  BgOpacity,
  BgOrigin,
  BgPosition,
  BgRepeat,
  BgSize,
  Blur,
  BorderCollapse,
  BorderColorB,
  BorderColorE,
  BorderColorL,
  BorderColorR,
  BorderColorS,
  BorderColorT,
  BorderColorX,
  BorderColorY,
  BorderColor,
  BorderOpacity,
  BorderSpacingX,
  BorderSpacingY,
  BorderSpacing,
  BorderStyle,
  BorderWB,
  BorderWE,
  BorderWL,
  BorderWR,
  BorderWS,
  BorderWT,
  BorderWX,
  BorderWY,
  BordeRW,
  Bottom,
  BoxDecoration,
  Box,
  BreakAfter,
  BreakBefore,
  BreakInside,
  Break,
  Brightness,
  Caption,
  CaretColor,
  Clear,
  ColEnd,
  ColStartEnd,
  ColStart,
  Columns,
  Container,
  Content,
  Contrast,
  Cursor,
  Delay,
  Display,
  DivideColor,
  DivideOpacity,
  DivideStyle,
  DivideXReverse,
  DivideX,
  DivideYReverse,
  DivideY,
  DropShadow,
  Duration,
  Ease,
  End,
  Fill,
  Filter,
  FlexDirection,
  FlexWrap,
  Flex,
  Float,
  FontFamily,
  FontSize,
  FontSmoothing,
  FontStyle,
  FontWeight,
  ForcedColorAdjust,
  FvnFigure,
  FvnFraction,
  FvnNormal,
  FvnOrdinal,
  FvnSlashedZero,
  FvnSpacing,
  GapX,
  GapY,
  Gap,
  GradientFromPos,
  GradientFrom,
  GradientToPos,
  GradientTo,
  GradientViaPos,
  GradientVia,
  Grayscale,
  GridCols,
  GridFlow,
  GridRows,
  Grow,
  H,
  HueRotate,
  Hyphens,
  Indent,
  InsetX,
  InsetY,
  Inset,
  Invert,
  Isolation,
  JustifyContent,
  JustifyItems,
  JustifySelf,
  Leading,
  Left,
  LineClamp,
  ListImage,
  ListStylePosition,
  ListStyleType,
  MH,
  Max,
  MaxW,
  MB,
  ME,
  MinH,
  MinW,
  MixBlend,
  ML,
  MR,
  MS,
  MT,
  MX,
  MY,
  ObjectFit,
  ObjectPosition,
  Opacity,
  Order,
  OutlineColor,
  OutlineOffset,
  OutlineStyle,
  OutlineW,
  OverflowX,
  OverflowY,
  Overflow,
  OverscrollX,
  OverscrollY,
  Overscroll,
  P,
  Pb,
  Pe,
  Pl,
  PlaceContent,
  PlaceItems,
  PlaceSelf,
  PlaceholderColor,
  PlaceholderOpacity,
  PointerEvents,
  Position,
  Pr,
  Ps,
  Pt,
  Px,
  Py,
  Resize,
  Right,
  RingColor,
  RingOffsetColor,
  RingOffsetW,
  RingOpacity,
  RingWInset,
  RingW,
  Rotate,
  RoundedB,
  RoundedBl,
  RoundedBr,
  RoundedE,
  RoundedEe,
  RoundedEs,
  RoundedL,
  RoundedR,
  RoundedS,
  RoundedSe,
  RoundedSs,
  RoundedT,
  RoundedTl,
  RoundedTr,
  Rounded,
  RowEnd,
  RowStartEnd,
  RowStart,
  Saturate,
  ScaleX,
  ScaleY,
  Scale,
  ScrollBehavior,
  ScrolM,
  ScrollMb,
  ScrollMe,
  ScrollMl,
  ScrollMr,
  ScrollMs,
  ScrollMt,
  ScrollMx,
  ScrollMy,
  ScrollP,
  ScrollPb,
  ScrollPe,
  ScrollPl,
  ScrollPr,
  ScrollPs,
  ScrollPt,
  ScrollPx,
  ScrollPy,
  Select,
  Sepia,
  ShadowColor,
  Shadow,
  Shrink,
  Size,
  SkewX,
  SkewY,
  SnapAlign,
  SnapStop,
  SnapStrictness,
  SnapType,
  SpaceXReverse,
  SpaceX,
  SpaceYReverse,
  SpaceY,
  Sr,
  Start,
  StrokeW,
  Stroke,
  TableLayout,
  TextAlignment,
  TextColor,
  TextDecorationColor,
  TextDecorationStyle,
  TextDecorationThickness,
  TextDecoration,
  TextOpacity,
  TextOverflow,
  TextTransform,
  TextWrap,
  Top,
  TouchPz,
  TouchX,
  TouchY,
  Touch,
  Tracking,
  TransformOrigin,
  Transform,
  Transition,
  TranslateX,
  TranslateY,
  UnderlineOffset,
  VerticalAlign,
  Visibility,
  W,
  Whitespace,
  WillChange,
  Z
}

