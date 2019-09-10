use bbclash::bbcode_to_html;

#[test]
fn empty_string() {
    assert_eq!(bbcode_to_html(""), 
    	"");
}
#[test]
fn no_tags() {
    assert_eq!(bbcode_to_html("I have no tags!"), 
    	"<p>I have no tags!</p>");
}
#[test]
fn one_tag() {
    assert_eq!(bbcode_to_html("I'm [b]bold![/b]"), 
    	"<p>I&#x27m <b>bold!</b></p>");
}
#[test]
fn two_tags() {
	assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
}
#[test]
fn nested_tags() {
	assert_eq!(bbcode_to_html("I'm [i][b]both italic and bold![/b][/i]"), 
		"<p>I&#x27m <i><b>both italic and bold!</b></i></p>");
}
#[test]
fn partially_nested_tags() {
	assert_eq!(bbcode_to_html("I'm [i]partly italic [b]and bold![/b][/i]"), 
		"<p>I&#x27m <i>partly italic <b>and bold!</b></i></p>");
}
#[test]
fn improperly_nested_tags() {
	assert_eq!(bbcode_to_html("I'm [i][b]fucking[/i] broken![/b]"), 
		"<p>I&#x27m <i><b>fucking</b></i><b> broken!</b></p>");
}
#[test]
fn missing_close_tag() {
	assert_eq!(bbcode_to_html("I'm [b]missing a closing tag!"), 
		"<p>I&#x27m <b>missing a closing tag!</b></p><b></b>");
}
#[test]
fn missing_open_tag() {
	assert_eq!(bbcode_to_html("I'm missing an opening[/i] tag!"), 
		"<p>I&#x27m missing an opening tag!</p>");
}
#[test]
fn empty_tag() {
	assert_eq!(bbcode_to_html("I have an [i][/i]empty tag!"), 
		"<p>I have an empty tag!</p>");
}
#[test]
fn void_tag() {
	assert_eq!(bbcode_to_html("I have an hr[hr] tag!"), 
		"<p>I have an hr</p><hr><p> tag!</p>");
}
#[test]
fn multiple_opening_tags() {
	assert_eq!(bbcode_to_html("I have several [b][b][b]opening tags, but only one closing![/b]"), 
		"<p>I have several <b><b><b>opening tags, but only one closing!</b></b></b></p><b><b></b></b>");
}
#[test]
fn multiple_closing_tags() {
	assert_eq!(bbcode_to_html("I have several [b]closing tags, but only one opening![/b][/b][/b]"), 
		"<p>I have several <b>closing tags, but only one opening!</b></p>");
}
#[test]
fn nested_missing_open_tag() {
	assert_eq!(bbcode_to_html("[b]I'm missing an opening[/i] tag![/b]"), 
		"<p><b>I&#x27m missing an opening</b><b> tag!</b></p>");
}
#[test]
fn bad_tag() {
	assert_eq!(bbcode_to_html("I have a [nonexistant]tag[/nonexistant]"), 
		"<p>I have a [nonexistant]tag[/nonexistant]</p>");
}
#[test]
fn single_newline() {
	assert_eq!(bbcode_to_html("I have a single\n newline!"), 
		"<p>I have a single<br>newline!</p>");
}
#[test]
fn double_newline() {
	assert_eq!(bbcode_to_html("I have a double\n\n newline!"), 
		"<p>I have a double</p><p>newline!</p>");
}
#[test]
fn triple_newline() {
	assert_eq!(bbcode_to_html("I have a triple\n\n\n newline!"), 
		"<p>I have a triple<br><br><br> newline!</p>");
}
#[test]
fn not_a_tag() {
	assert_eq!(bbcode_to_html("This is [not a tag], just some [square] brackets!"), 
		"<p>This is [not a tag], just some [square] brackets!</p>");
}