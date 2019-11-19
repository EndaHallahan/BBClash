use bbclash::bbcode_to_html;
use bbclash::bbcode_to_html_ugly;

/*-- COLOUR --*/
#[test]
fn color_no_argument() {
    assert_eq!(bbcode_to_html("[color]This should not be coloured[/color]"), 
    	"<p>This should not be coloured</p>");
}
#[test]
fn color_name_arg() {
    assert_eq!(bbcode_to_html("[color=red]This should be red[/color]"), 
    	"<p><span style=\"color:red;\">This should be red</span></p>");
}
#[test]
fn color_hex_arg() {
    assert_eq!(bbcode_to_html("[color=#FF0000]This should be red[/color]"), 
    	"<p><span style=\"color:#FF0000;\">This should be red</span></p>");
}
#[test]
fn color_name_bad_arg() {
    assert_eq!(bbcode_to_html("[color=talapia]This should be broken[/color]"), 
    	"<p>This should be broken</p>");
}
#[test]
fn color_hex_bad_arg() {
    assert_eq!(bbcode_to_html("[color=#$0ffdddd]This should be broken[/color]"), 
    	"<p>This should be broken</p>");
}
#[test]
fn color_name_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[color=talapia]This should be broken[/color]"), 
        "<p>[color=talapia]This should be broken[/color]</p>");
}
#[test]
fn color_hex_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[color=#$0ffdddd]This should be broken[/color]"), 
        "<p>[color=#$0ffdddd]This should be broken[/color]</p>");
}
#[test]
fn color_no_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[color]This should not be coloured[/color]"), 
        "<p>[color]This should not be coloured[/color]</p>");
}
#[test]
fn colour_no_argument() {
    assert_eq!(bbcode_to_html("[colour]This should not be coloured[/colour]"), 
        "<p>This should not be coloured</p>");
}
#[test]
fn colour_name_arg() {
    assert_eq!(bbcode_to_html("[colour=red]This should be red[/colour]"), 
        "<p><span style=\"color:red;\">This should be red</span></p>");
}
#[test]
fn colour_hex_arg() {
    assert_eq!(bbcode_to_html("[colour=#FF0000]This should be red[/colour]"), 
        "<p><span style=\"color:#FF0000;\">This should be red</span></p>");
}
#[test]
fn colour_name_bad_arg() {
    assert_eq!(bbcode_to_html("[colour=talapia]This should be broken[/colour]"), 
        "<p>This should be broken</p>");
}
#[test]
fn colour_hex_bad_arg() {
    assert_eq!(bbcode_to_html("[colour=#$0ffdddd]This should be broken[/colour]"), 
        "<p>This should be broken</p>");
}
#[test]
fn colour_name_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[colour=talapia]This should be broken[/colour]"), 
        "<p>[colour=talapia]This should be broken[/colour]</p>");
}
#[test]
fn colour_hex_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[colour=#$0ffdddd]This should be broken[/colour]"), 
        "<p>[colour=#$0ffdddd]This should be broken[/colour]</p>");
}
#[test]
fn colour_no_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[colour]This should not be coloured[/colour]"), 
        "<p>[colour]This should not be coloured[/colour]</p>");
}

/*-- URL --*/
#[test]
fn url_no_argument() {
    assert_eq!(bbcode_to_html("[url]https://www.penclash.com[/url]"), 
    	"<p><a href=\"https://www.penclash.com\" rel=\"nofollow\">https://www.penclash.com</a></p>");
}
#[test]
fn url_no_argument_no_text() {
    assert_eq!(bbcode_to_html("[url][/url]This should not be a link."), 
        "<p>This should not be a link.</p>");
}
#[test]
fn url_https_arg() {
    assert_eq!(bbcode_to_html("[url=https://www.penclash.com]This should be a link[/url]"), 
    	"<p><a href=\"https://www.penclash.com\" rel=\"nofollow\">This should be a link</a></p>");
}
#[test]
fn url_http_arg() {
    assert_eq!(bbcode_to_html("[url=http://www.penclash.com]This should be a link[/url]"), 
    	"<p><a href=\"http://www.penclash.com\" rel=\"nofollow\">This should be a link</a></p>");
}
#[test]
fn url_www_arg() {
    assert_eq!(bbcode_to_html("[url=www.penclash.com]This should be a link[/url]"), 
    	"<p><a href=\"http://www.penclash.com\" rel=\"nofollow\">This should be a link</a></p>");
}
#[test]
fn url_no_text() {
    assert_eq!(bbcode_to_html("[url][/url]This should not be a link."), 
        "<p>This should not be a link.</p>");
}
#[test]
fn url_forbidden_char_arg() {
    assert_eq!(bbcode_to_html("[url=alert(\"Hacked!\");]This should not be a link[/url]"), 
    	"<p>This should not be a link</p>");
}
#[test]
fn url_bad_arg() {
    assert_eq!(bbcode_to_html("[url=javascript:get_ganked.js]This should not be a link[/url]"), 
    	"<p>This should not be a link</p>");
}
#[test]
fn url_forbidden_char_no_arg() {
    assert_eq!(bbcode_to_html("[url]alert(\"Hacked!\");[/url]"), 
        "");
}
#[test]
fn url_bad_no_arg() {
    assert_eq!(bbcode_to_html("[url]javascript:get_ganked.js[/url]"), 
        "");
}
#[test]
fn url_no_text_ugly() {
    assert_eq!(bbcode_to_html_ugly("[url][/url]This should not be a link."), 
        "<p>[url][/url]This should not be a link.</p>");
}
#[test]
fn url_forbidden_char_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[url=alert(\"Hacked!\");]This should not be a link[/url]"), 
        "<p>[url=alert(&quotHacked!&quot);]This should not be a link[/url]</p>");
}
#[test]
fn url_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[url=javascript:get_ganked.js]This should not be a link[/url]"), 
        "<p>[url=javascript:get_ganked.js]This should not be a link[/url]</p>");
}
#[test]
fn url_forbidden_char_no_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[url]alert(\"Hacked!\");[/url]"), 
        "<p>[url]alert(&quotHacked!&quot);[/url]</p>");
}
#[test]
fn url_bad_no_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[url]javascript:get_ganked.js[/url]"), 
        "<p>[url]javascript:get_ganked.js[/url]</p>");
}

/*-- OPACITY --*/
#[test]
fn opacity_no_argument() {
    assert_eq!(bbcode_to_html("[opacity]This should not be transparant![/opacity]"), 
    	"<p>This should not be transparant!</p>");
}
#[test]
fn opacity_bad_argument() {
    assert_eq!(bbcode_to_html("[opacity=fish]This should not be transparant![/opacity]"), 
    	"<p>This should not be transparant!</p>");
}
#[test]
fn opacity_no_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[opacity]This should not be transparant![/opacity]"), 
        "<p>[opacity]This should not be transparant![/opacity]</p>");
}
#[test]
fn opacity_bad_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[opacity=fish]This should not be transparant![/opacity]"), 
        "<p>[opacity=fish]This should not be transparant![/opacity]</p>");
}
#[test]
fn opacity_argument() {
    assert_eq!(bbcode_to_html("[opacity=.3]This should be transparant![/opacity]"), 
    	"<p><span style=\"opacity:0.3;\">This should be transparant!</span></p>");
}
#[test]
fn opacity_perc_argument() {
    assert_eq!(bbcode_to_html("[opacity=30%]This should be transparant![/opacity]"), 
    	"<p><span style=\"opacity:0.3;\">This should be transparant!</span></p>");
}

/*-- SIZE --*/
#[test]
fn size_no_argument() {
    assert_eq!(bbcode_to_html("[size]This should be normal![/size]"), 
    	"<p>This should be normal!</p>");
}
#[test]
fn size_bad_argument() {
    assert_eq!(bbcode_to_html("[size=fish]This should be normal![/size]"), 
    	"<p>This should be normal!</p>");
}
#[test]
fn size_no_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[size]This should be normal![/size]"), 
        "<p>[size]This should be normal![/size]</p>");
}
#[test]
fn size_bad_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[size=fish]This should be normal![/size]"), 
        "<p>[size=fish]This should be normal![/size]</p>");
}
#[test]
fn size_argument() {
    assert_eq!(bbcode_to_html("[size=8]This should be small![/size]"), 
    	"<p><span style=\"font-size:0.5rem;\">This should be small!</span></p>");
}
#[test]
fn size_over_argument() {
    assert_eq!(bbcode_to_html("[size=40]This should be 2em![/size]"), 
    	"<p><span style=\"font-size:2rem;\">This should be 2em!</span></p>");
}
#[test]
fn size_under_argument() {
    assert_eq!(bbcode_to_html("[size=1]This should be .5em![/size]"), 
    	"<p><span style=\"font-size:0.5rem;\">This should be .5em!</span></p>");
}
#[test]
fn size_em_argument() {
    assert_eq!(bbcode_to_html("[size=.7em]This should be small![/size]"), 
    	"<p><span style=\"font-size:0.7rem;\">This should be small!</span></p>");
}
#[test]
fn size_em_over_argument() {
    assert_eq!(bbcode_to_html("[size=3em]This should be 2em![/size]"), 
    	"<p><span style=\"font-size:2rem;\">This should be 2em!</span></p>");
}
#[test]
fn size_em_under_argument() {
    assert_eq!(bbcode_to_html("[size=.1em]This should be .5em![/size]"), 
    	"<p><span style=\"font-size:0.5rem;\">This should be .5em!</span></p>");
}

/*-- IMAGE --*/
#[test]
fn image_no_argument() {
    assert_eq!(bbcode_to_html("[img][/img]"), 
        "");
}
#[test]
fn image_https_arg() {
    assert_eq!(bbcode_to_html("[img]https://endahallahan.github.io/Penclash-Splash-Site/resources/logo.png[/img]"), 
        "<p><img src=\"https://endahallahan.github.io/Penclash-Splash-Site/resources/logo.png\"></p>");
}
#[test]
fn image_http_arg() {
    assert_eq!(bbcode_to_html("[img]http://endahallahan.github.io/Penclash-Splash-Site/resources/logo.png[/img]"), 
        "<p><img src=\"http://endahallahan.github.io/Penclash-Splash-Site/resources/logo.png\"></p>");
}
#[test]
fn image_www_arg() {
    assert_eq!(bbcode_to_html("[img]www.endahallahan.github.io/Penclash-Splash-Site/resources/logo.png[/img]"), 
        "<p><img src=\"http://www.endahallahan.github.io/Penclash-Splash-Site/resources/logo.png\"></p>");
}
#[test]
fn image_bad_filetype() {
    assert_eq!(bbcode_to_html("[img]https://d/bad_image.svg[/img]"), 
        "");
}
#[test]
fn image_bad_arg() {
    assert_eq!(bbcode_to_html("[img]a onerror=alert('XSS')[/img]"), 
        "");
}
#[test]
fn image_no_argument_ugly() {
    assert_eq!(bbcode_to_html_ugly("[img][/img]"), 
        "<p>[img][/img]</p>");
}
#[test]
fn image_bad_filetype_ugly() {
    assert_eq!(bbcode_to_html_ugly("[img]https://d/bad_image.svg[/img]"), 
        "<p>[img]https://d/bad_image.svg[/img]</p>");
}
#[test]
fn image_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[img]a onerror=alert('XSS')[/img]"), 
        "<p>[img]a onerror=alert(&#x27XSS&#x27)[/img]</p>");
}

/*-- QUOTE --*/
#[test]
fn quote_no_argument() {
    assert_eq!(bbcode_to_html("[quote]To be, or not to be.[/quote]"), 
        "<blockquote><p>To be, or not to be.</p></blockquote>");
}
#[test]
fn quote_argument() {
    assert_eq!(bbcode_to_html("[quote=Shakespeare]To be, or not to be.[/quote]"), 
        "<blockquote data-author=\"Shakespeare\"><p>To be, or not to be.</p></blockquote>");
}
#[test]
fn quote_multiline() {
    assert_eq!(bbcode_to_html("[quote]To be, or not to be.

        That is the question.

        [/quote]"), 
        "<blockquote><p>To be, or not to be.</p><p>That is the question.</p></blockquote>");
}
#[test]
fn empty_quote() {
    assert_eq!(bbcode_to_html("[quote][/quote]"), 
        "");
}
#[test]
fn empty_quote_ugly() {
    assert_eq!(bbcode_to_html_ugly("[quote][/quote]"), 
        "<blockquote></blockquote>");
}

/*-- HEADER --*/
#[test]
fn h1() {
    assert_eq!(bbcode_to_html("[h1]To be, or not to be.[/h1]"), 
        "<h1>To be, or not to be.</h1>");
}
#[test]
fn h2() {
    assert_eq!(bbcode_to_html("[h2]To be, or not to be.[/h2]"), 
        "<h2>To be, or not to be.</h2>");
}
#[test]
fn h3() {
    assert_eq!(bbcode_to_html("[h3]To be, or not to be.[/h3]"), 
        "<h3>To be, or not to be.</h3>");
}
#[test]
fn h4() {
    assert_eq!(bbcode_to_html("[h4]To be, or not to be.[/h4]"), 
        "<h4>To be, or not to be.</h4>");
}
#[test]
fn h5() {
    assert_eq!(bbcode_to_html("[h5]To be, or not to be.[/h5]"), 
        "<h5>To be, or not to be.</h5>");
}
#[test]
fn h6() {
    assert_eq!(bbcode_to_html("[h6]To be, or not to be.[/h6]"), 
        "<h6>To be, or not to be.</h6>");
}
#[test]
fn header_interrupt() {
    assert_eq!(bbcode_to_html("That is the [h1]To be, or not to be.[/h1] question."), 
        "<p>That is the </p><h1>To be, or not to be.</h1><p> question.</p>");
}
#[test]
fn header_adjacent() {
    assert_eq!(bbcode_to_html("[h1]To be, or not to be.[/h1][h2]That is the question.[/h2]"), 
        "<h1>To be, or not to be.</h1><h2>That is the question.</h2>");
}
#[test]
fn header_adjacent_paragraph() {
    assert_eq!(bbcode_to_html("[h1]To be, or not to be.[/h1]

        That is the question."), 
        "<h1>To be, or not to be.</h1><p>That is the question.</p>");
}
#[test]
fn bad_header_level() {
    assert_eq!(bbcode_to_html("[h7]To be, or not to be.[/h7]"), 
        "<p>[h7]To be, or not to be.[/h7]</p>");
}

/*-- FOOTNOTE --*/
#[test]
fn footnote_no_arg() {
    assert_eq!(bbcode_to_html("To be[footnote]Or not to be[/footnote]. That is the question."), 
        "<p>To be<span class=\"footnote\">Or not to be</span>. That is the question.</p>");
}
#[test]
fn footnote_arg() {
    assert_eq!(bbcode_to_html("To be[footnote=*]Or not to be[/footnote]. That is the question."), 
        "<p>To be<span class=\"footnote\" data-symbol=\"*\">Or not to be</span>. That is the question.</p>");
}

/*-- PRE --*/
#[test]
fn pre() {
    assert_eq!(bbcode_to_html("[pre]To be, or not to be.[/pre]"), 
        "<pre>To be, or not to be.</pre>");
}
#[test]
fn pre_formatting() {
    assert_eq!(bbcode_to_html("[pre]To be, or 

        not\t to 
        \tbe.[/pre]"), 
        "<pre>To be, or \n\nnot\t to \n\tbe.</pre>");
}
#[test]
fn pre_interrupt() {
    assert_eq!(bbcode_to_html("That is the [pre]To be, or not to be.[/pre] question."), 
        "<p>That is the </p><pre>To be, or not to be.</pre><p> question.</p>");
}
#[test]
fn pre_adjacent() {
    assert_eq!(bbcode_to_html("[pre]To be, or not to be.[/pre][pre]That is the question.[/pre]"), 
        "<pre>To be, or not to be.</pre><pre>That is the question.</pre>");
}
#[test]
fn pre_adjacent_paragraph() {
    assert_eq!(bbcode_to_html("[pre]To be, or not to be.[/pre]

        That is the question."), 
        "<pre>To be, or not to be.</pre><p>That is the question.</p>");
}

/*-- CODE --*/
#[test]
fn code() {
    assert_eq!(bbcode_to_html("[code]To be, or not to be.[/code]"), 
        "<p><code>To be, or not to be.</code></p>");
}
#[test]
fn code_bbcode() {
    assert_eq!(bbcode_to_html("[code]To be, or [b]not[/b] to be.[/code]"), 
        "<p><code>To be, or [b]not[/b] to be.</code></p>");
}

/*-- CODEBLOCK --*/
#[test]
fn codeblock_no_arg() {
    assert_eq!(bbcode_to_html("[codeblock]To be, or not to be.[/codeblock]"), 
        "<pre>To be, or not to be.</pre>");
}
#[test]
fn codeblock_arg() {
    assert_eq!(bbcode_to_html("[codeblock=rust]To be, or not to be.[/codeblock]"), 
        "<pre data-language=\"rust\">To be, or not to be.</pre>");
}
#[test]
fn codeblock_interrupt() {
    assert_eq!(bbcode_to_html("That is the [codeblock]To be, or not to be.[/codeblock] question."), 
        "<p>That is the </p><pre>To be, or not to be.</pre><p> question.</p>");
}
#[test]
fn codeblock_adjacent() {
    assert_eq!(bbcode_to_html("[codeblock]To be, or not to be.[/codeblock][codeblock]That is the question.[/codeblock]"), 
        "<pre>To be, or not to be.</pre><pre>That is the question.</pre>");
}
#[test]
fn codeblock_adjacent_paragraph() {
    assert_eq!(bbcode_to_html("[codeblock]To be, or not to be.[/codeblock]

        That is the question."), 
        "<pre>To be, or not to be.</pre><p>That is the question.</p>");
}
#[test]
fn codeblock_bbcode() {
    assert_eq!(bbcode_to_html("[codeblock]To be, or [b]not[/b] to be.[/codeblock]"), 
        "<pre>To be, or [b]not[/b] to be.</pre>");
}

/*-- FIGURE --*/
#[test]
fn figure_right_arg() {
    assert_eq!(bbcode_to_html("[figure=right]To be, or not to be.[/figure]"), 
        "<figure class=\"figure-right\"><p>To be, or not to be.</p></figure>");
}
#[test]
fn figure_left_arg() {
    assert_eq!(bbcode_to_html("[figure=left]To be, or not to be.[/figure]"), 
        "<figure class=\"figure-left\"><p>To be, or not to be.</p></figure>");
}
#[test]
fn figure_bad_arg() {
    assert_eq!(bbcode_to_html("[figure=up]To be, or not to be.[/figure]"), 
        "<p>To be, or not to be.</p>");
}
#[test]
fn figure_interrupt() {
    assert_eq!(bbcode_to_html("That is the [figure=right]To be, or not to be.[/figure] question."), 
        "<p>That is the </p><figure class=\"figure-right\"><p>To be, or not to be.</p></figure><p> question.</p>");
}
#[test]
fn figure_adjacent() {
    assert_eq!(bbcode_to_html("[figure=right]To be, or not to be.[/figure][figure=right]That is the question.[/figure]"), 
        "<figure class=\"figure-right\"><p>To be, or not to be.</p></figure><figure class=\"figure-right\"><p>That is the question.</p></figure>");
}
#[test]
fn figure_adjacent_paragraph() {
    assert_eq!(bbcode_to_html("[figure=right]To be, or not to be.[/figure]

        That is the question."), 
        "<figure class=\"figure-right\"><p>To be, or not to be.</p></figure><p>That is the question.</p>");
}

/*-- LIST --*/
#[test]
fn simple_list() {
    assert_eq!(bbcode_to_html("[list][*]To be[*]or not to be[/list]"), 
        "<ul><li><p>To be</p></li><li><p>or not to be</p></li></ul>");
}
#[test]
fn expanded_simple_list() {
    assert_eq!(bbcode_to_html("
        [list]
            [*]To be
            [*]or not to be
        [/list]"), 
        "<ul><li><p>To be</p></li><li><p>or not to be</p></li></ul>");
}
#[test]
fn styled_ordered_list() {
    assert_eq!(bbcode_to_html("
        [list=1]
            [*]To be
            [*]or not to be
        [/list]"), 
        "<ol type=\"1\"><li><p>To be</p></li><li><p>or not to be</p></li></ol>");
}
#[test]
fn styled_unordered_list() {
    assert_eq!(bbcode_to_html("
        [list=circle]
            [*]To be
            [*]or not to be
        [/list]"), 
        "<ul style=\"list-style-type:circle;\"><li><p>To be</p></li><li><p>or not to be</p></li></ul>");
}
#[test]
fn nested_simple_list() {
    assert_eq!(bbcode_to_html("
        [list]
            [*]To be
            [*]or not to be
            [*]That [list]
                [*]is the
                [*]question
                [/list]
        [/list]"), 
        "<ul><li><p>To be</p></li><li><p>or not to be</p></li><li><p>That </p><ul><li><p>is the</p></li><li><p>question</p></li></ul></li></ul>");
}
#[test]
fn styled_list_bad_arg() {
    assert_eq!(bbcode_to_html("
        [list=fish]
            [*]To be
            [*]or not to be
        [/list]"), 
        "<p><br><br>To be<br>or not to be</p>");
}
#[test]
fn list_no_list() {
    assert_eq!(bbcode_to_html("
        [*]To be
        [*]or not to be"), 
        "<p><br>To be<br>or not to be</p>");
}
#[test]
fn styled_list_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("
        [list=fish]
            [*]To be
            [*]or not to be
        [/list]"), 
        "<p><br>[list=fish]<br>[*]To be<br>[*]or not to be[/list]</p>");
}
#[test]
fn list_no_list_ugly() {
    assert_eq!(bbcode_to_html_ugly("
        [*]To be
        [*]or not to be"), 
        "<p><br>[*]To be<br>[*]or not to be</p>");
}

/*-- TABLE --*/
#[test]
fn table() {
    assert_eq!(bbcode_to_html("
        [table]
            [caption]Macbeth[/caption]
            [tr]
                [th]To be[/th]
                [th]or not to be[/th]
            [/tr]
            [tr]
                [td]that is[/td]
                [td]the question[/td]
            [/tr]
        [/table]"), 
        "<table><caption><p>Macbeth</p></caption><tr><th><p>To be</p></th><th><p>or not to be</p></th></tr><tr><td><p>that is</p></td><td><p>the question</p></td></tr></table>");
}
#[test]
fn table_no_table_rows() {
    assert_eq!(bbcode_to_html("
        [th]To be[/th]
        [th]or not to be[/th]
        [td]that is[/td]
        [td]the question[/td]"), 
        "<p><br>To be<br>or not to be<br>that is<br>the question</p>");
}
#[test]
fn table_no_table() {
    assert_eq!(bbcode_to_html("
        [caption]Macbeth[/caption]
        [tr]
            [th]To be[/th]
            [th]or not to be[/th]
        [/tr]
        [tr]
            [td]that is[/td]
            [td]the question[/td]
        [/tr]"), 
        "<p><br>Macbeth<br><br>To be<br>or not to be<br><br>that is<br>the question</p>");
}
#[test]
fn table_no_table_ugly() {
    assert_eq!(bbcode_to_html_ugly("
        [caption]Macbeth[/caption]
        [tr]
            [th]To be[/th]
            [th]or not to be[/th]
        [/tr]
        [tr]
            [td]that is[/td]
            [td]the question[/td]
        [/tr]"), 
        "<p><br>[caption]Macbeth[/caption]<br>[tr]<br>[th]To be[/th]<br>[th]or not to be[/th][/tr]<br>[tr]<br>[td]that is[/td]<br>[td]the question[/td][/tr]</p>");
}

/*-- PRE-LINE --*/

/*-- INDENT --*/
#[test]
fn indent_no_arg() {
    assert_eq!(bbcode_to_html("[indent]To be, or not to be.[/indent]"), 
        "<div class=\"indent-1\"><p>To be, or not to be.</p></div>");
}
#[test]
fn indent_arg() {
    assert_eq!(bbcode_to_html("[indent=3]To be, or not to be.[/indent]"), 
        "<div class=\"indent-3\"><p>To be, or not to be.</p></div>");
}
#[test]
fn indent_bad_arg() {
    assert_eq!(bbcode_to_html("[indent=7]To be, or not to be.[/indent]"), 
        "<p>To be, or not to be.</p>");
}
#[test]
fn indent_bad_arg_ugly() {
    assert_eq!(bbcode_to_html_ugly("[indent=7]To be, or not to be.[/indent]"), 
        "<p>[indent=7]To be, or not to be.[/indent]</p>");
}

/*-- MATH --*/
#[test]
fn math() {
    assert_eq!(bbcode_to_html("[math]Some TeX code[/math]"), 
        "<p><span class=\"math_container\">Some TeX code</span></p>");
}

/*-- MATHBLOCK --*/
#[test]
fn mathblock() {
    assert_eq!(bbcode_to_html("[mathblock]Some TeX code[/mathblock]"), 
        "<div class=\"math_container\">Some TeX code</div>");
}

/*-- EMBED --*/
#[test]
fn embed() {
    assert_eq!(bbcode_to_html("[embed]https://www.penclash.com[/embed]"), 
        "<div class=\"embed\" data-content=\"https://www.penclash.com\"></div>");
}

/*-- EMAIL --*/
#[test]
fn email() {
    assert_eq!(bbcode_to_html("[email]test@penclash.com[/email]"), 
        "<p><a href=\"mailto:test@penclash.com\">test@penclash.com</a></p>");
}
#[test]
fn email_no_text() {
    assert_eq!(bbcode_to_html("[email][/email]This should not be a link."), 
        "<p>This should not be a link.</p>");
}
#[test]
fn email_no_text_ugly() {
    assert_eq!(bbcode_to_html_ugly("[email][/email]This should not be a link."), 
        "<p>[email][/email]This should not be a link.</p>");
}