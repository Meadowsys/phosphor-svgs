#[inline]
pub fn duotone_opacity(icon: &'static str, opacity: f64) -> String {
	const DEFAULT_OPACITY: &str = r#"opacity="0.2""#;
	debug_assert!((0.0..=1.0).contains(&opacity));
	debug_assert!(icon.contains(DEFAULT_OPACITY));
	icon.replacen(DEFAULT_OPACITY, &format!(r#"opacity="{opacity}""#), 1)
}
