pub fn get_config() -> String {
    String::from(
        r#"[themes]
syntax = "Tomorrow-Night.tmTheme"

[tagging]
sorted = true
"#,
    )
}

pub fn get_theme() -> String {
    String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>comment</key>
	<string>http://chriskempson.com</string>
	<key>name</key>
	<string>Tomorrow Night</string>
	<key>settings</key>
	<array>
		<dict>
			<key>settings</key>
			<dict>
				<key>background</key>
				<string>#1D1F21</string>
				<key>caret</key>
				<string>#AEAFAD</string>
				<key>foreground</key>
				<string>#C5C8C6</string>
				<key>invisibles</key>
				<string>#4B4E55</string>
				<key>lineHighlight</key>
				<string>#282A2E</string>
				<key>selection</key>
				<string>#373B41</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Comment</string>
			<key>scope</key>
			<string>comment</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#969896</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Foreground</string>
			<key>scope</key>
			<string>keyword.operator.class, constant.other, source.php.embedded.line</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#CED1CF</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Variable, String Link, Regular Expression, Tag Name, GitGutter deleted</string>
			<key>scope</key>
			<string>variable, support.other.variable, string.other.link, string.regexp, entity.name.tag, entity.other.attribute-name, meta.tag, declaration.tag, markup.deleted.git_gutter</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#CC6666</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Number, Constant, Function Argument, Tag Attribute, Embedded</string>
			<key>scope</key>
			<string>constant.numeric, constant.language, support.constant, constant.character, variable.parameter, punctuation.section.embedded, keyword.other.unit</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#DE935F</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Class, Support</string>
			<key>scope</key>
			<string>entity.name.class, entity.name.type.class, support.type, support.class</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#F0C674</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>String, Symbols, Inherited Class, Markup Heading, GitGutter inserted</string>
			<key>scope</key>
			<string>string, constant.other.symbol, entity.other.inherited-class, entity.name.filename, markup.heading, markup.inserted.git_gutter</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#B5BD68</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Operator, Misc</string>
			<key>scope</key>
			<string>keyword.operator, constant.other.color</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#8ABEB7</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Function, Special Method, Block Level, GitGutter changed</string>
			<key>scope</key>
			<string>entity.name.function, meta.function-call, support.function, keyword.other.special-method, meta.block-level, markup.changed.git_gutter</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#81A2BE</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Keyword, Storage</string>
			<key>scope</key>
			<string>keyword, storage, storage.type, entity.name.tag.css</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#B294BB</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Invalid</string>
			<key>scope</key>
			<string>invalid</string>
			<key>settings</key>
			<dict>
				<key>background</key>
				<string>#DF5F5F</string>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#CED2CF</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Separator</string>
			<key>scope</key>
			<string>meta.separator</string>
			<key>settings</key>
			<dict>
				<key>background</key>
				<string>#82A3BF</string>
				<key>foreground</key>
				<string>#CED2CF</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Deprecated</string>
			<key>scope</key>
			<string>invalid.deprecated</string>
			<key>settings</key>
			<dict>
				<key>background</key>
				<string>#B798BF</string>
				<key>fontStyle</key>
				<string></string>
				<key>foreground</key>
				<string>#CED2CF</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Diff foreground</string>
			<key>scope</key>
			<string>markup.inserted.diff, markup.deleted.diff, meta.diff.header.to-file, meta.diff.header.from-file</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#FFFFFF</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Diff insertion</string>
			<key>scope</key>
			<string>markup.inserted.diff, meta.diff.header.to-file</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#718c00</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Diff deletion</string>
			<key>scope</key>
			<string>markup.deleted.diff, meta.diff.header.from-file</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#c82829</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Diff header</string>
			<key>scope</key>
			<string>meta.diff.header.from-file, meta.diff.header.to-file</string>
			<key>settings</key>
			<dict>
				<key>foreground</key>
				<string>#FFFFFF</string>
				<key>background</key>
				<string>#4271ae</string>
			</dict>
		</dict>
		<dict>
			<key>name</key>
			<string>Diff range</string>
			<key>scope</key>
			<string>meta.diff.range</string>
			<key>settings</key>
			<dict>
				<key>fontStyle</key>
				<string>italic</string>
				<key>foreground</key>
				<string>#3e999f</string>
			</dict>
		</dict>
	</array>
	<key>uuid</key>
	<string>F96223EB-1A60-4617-92F3-D24D4F13DB09</string>
  <key>colorSpaceName</key>
  <string>sRGB</string>
</dict>
</plist>
"#,
    )
}

pub fn get_page_template() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title></title>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link href="../assets/css/style.css" rel="stylesheet">
    </head>
    <body>
        <div class="wrapper">
			{{ content }}
        </div>
    </body>
</html>
"#,
    )
}

pub fn get_index_template() -> String {
    String::from(
        r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Index</title>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <link href="assets/css/style.css" rel="stylesheet">
    </head>
    <body>
        <div class="wrapper">
            <h1>Welcome to your website</h1>
            <p>This is the default homepage generated for you.</p>
            <p>You can customize it to your liking! Look at your first page:</p>

			{{ generic_post }}
        </div>
    </body>
</html>
"#,
    )
}

pub fn get_css() -> String {
    String::from(
        r#".wrapper {
    display: block;
    margin: 0 auto;
    max-width: 600px;
}
"#,
)
    }

pub fn get_generic_post() -> String {
    String::from(
        r#"---
title: First Post
tags: generic_post
---

# Your first post

This is a page inside the `content/` directory.

It is written in markdown.

### Further Customization

You can modify the `page.html` template inside the `templates/` directory if you wish to add things to all pages like a header, navbar, footer, etc.
"#,
)
    }
