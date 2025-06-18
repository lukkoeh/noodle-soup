# UI - DOCS
This file is meant to provide an overview on our *custom components* and their *properties* and *callbacks*

## globals

### Theme defined in *ui/base-components/theme-struct.slint*

**in-out properties**
- fs-h1 > length defines height of Large Headlines defaults to 28px
- fs-h2: length defines height of medium headlines defaults to 22px
- fs-l > length defines large text / icons defaults to 16px
- fs-m > length defines size of normal text defaults to 14px
- fs-s > length defines size of small text / tags defaults to 12px

- c-txt-norm > brush color for normal text defaults to #000
- c-txt-search > brush color for searchfield text defaults to #898989
- c-bg-main > brush color for general background defaults to #FFF
- c-bg-widget > brush color for widget background defaults to #FFF
- c-bg-input > brush color for search and dropdown background defaults to #E0E0E0

- c-m-1 > brush color for horizontal line defaults to #CACACA

- c-ac > brush accent color -20 lightness in darkmode defaults to #ff0000
- c-ac-trans > brush accent color 20% transparency to #ff0000aa
- c-ac-light > brush accent color +20 lightness defaults to #ff0000

- pad > padding for text and button elements defaults to 5px

## Basic components

### UiButton defined in *ui/base-components/ui-button.slint*

**input properties**
- type > ButtonTypes enum for different versions of the Button (primary, secondary, round, simple)
- text > string display text for the Button
- icon > string intended for unicode strings

**output properties**
- has-hover > bool is true when button has hover

**callbacks**
- clicked > is called when user clicks or touches the Button


### InputLine defined in *ui/base-components/input-line.slint*

**input properties**
- placeholder > string is only visible when the Input is empty also used for the accesibility placeholder
- icon > string optional displays icon button at the end of the input line
- font_size > length determins fontsize defaults to Theme.fs-m

**in-out properties**
- input_text > string contains the text of the input can be set via code for edits

**callbacks**
- input-action > is called when the button at the end is clicked


### IconFA defined in *ui/base-components/fa-icon.slint*

**input properties**
- icon > string should be a unicode string in the format "\u{f002}". All possible Icons can be found [here](https://fontawesome.com/v4/icons/).
- icon-size > length determins how large the Icon should be


### Tooltip defined in *ui/base-components/tooltip.slint*

**input properties**
-
**output properties**
-

**functions**


### TagBubble defined in *ui/base-components/tag-bubble.slint*

**input properties**
- text_content > string will be displayed in the tag
- font_height > length defaults to Theme.fs-s
- bg_color > brush defines background color defaults to Theme.c-ac-trans
- font_color: brush defaults to Theme.c-ac


## Basic Widgets

### Popover