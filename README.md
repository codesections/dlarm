# dlarm

dlarm is the alarm system for [dwm](https://dwm.suckless.org/).  

Like dwm, dlarm is minimalist, keyboard-focused, and terminal centric.  All dlarm does is to allow you to set an alarm for a specified time later today.  When that alarm goes off, your dwm status bar will flash with either a string you specify or the text `ALARM ALARM ALARM`.  You can then turn off or snooze the alarm.

It looks like this:

[Gif depicting dlarm alarm sounding](./dlarm.gif)

# Installation prerequisites

###dwm
dlarm requires dwm.  If you do not already have it installed, you can install it from [dwm.suckless.org](https://dwm.suckless.org/).  If you would like dlarm to support a different minimalist widow manager, please open an issue and let me know.

###dwm-statuscolors (optional)
dlarm works best with the dwm-statuscolors patch applied.  This patch allows color output in dwm's status bar; without this patch, dlarm will still function but will not have any colors.  If you do not already have this patch, you can install it from [dwm.suckless.org/patches/statuscolors](https://dwm.suckless.org/patches/statuscolors/).

###rust
At the moment, dlarm requires the Rust tool chain to be installed on your system.  If you would like a pre-compiled binary, please open an issue and let me know your target architecture.

# Installation Instructions

### Install dlarm
Install dlarm using Cargo with `cargo install dlarm`.

### Install man(1) page and zsh completions (optional)
Clone this repository and run `sudo make install` to automatically install the man page and zsh completions.

### Install other shell completions (optional)
If you use a different shell, you can generate the completion script for your shell with the generate command (e.g., `dlarm generate bash`).  You may also use this command to generate the man page if you prefer.  dlarm supports bash, elvish, fish, and zsh completions.

### Edit your .xinitrc or other file to enable .dlarm
Finally, edit whatever file contains the script that sets your dwm status bar.  Typically, this is `.xinitrc` or some other init script.  It should contain some sort of script on an infinite loop that updates your dwm status bar info (for example, with a clock).  

Once you have located that script, paste in the appropriate script from the dlarm repository.  If you use dwm-statuscolors, then you should use `script_for_color.sh`; if not, you should use `script_without_color.sh`.  As indicated in those files, you should move the pre-existing script into the `else` block in the new script—that way, the alarm will be displayed when it is triggered, and your normal status info will be displayed at every other time.

Optionally, if you use dwm-statuscolors, you can edit the script to select which colors are displayed when the alarm alerts.  Recall that you specify a color for dwm-statuscolors by specifying the binary value that corresponds to the order you defined the colors in your `config.h` (e.g., `\x03` for the third color you defined).


## Usage

You interact with dlarm with the `--set`, `--message`, `--snooze`, and `--off` commands.  Consult the `--help` command or the man(1) page for additional usage details.

When setting an alarm, you must pass in the time in either `H:MMpp` format (e.g., `1:35pm`) or `H:MM` format (e.g., `11:30`).  If you do not include a period (am/pm), then dlarm will guess at the period by setting the alarm for an AM time if that time would be in the future or else a PM time.


## Goals and non-goals

My goal is for dlarm to be simple, minimalist, and user-configurable.  I plan to continue to develop dlarm; future plans include allowing for current-time relative alarms (e.g. `dlarm --set +25` to set an alarm for 25 minutes from now).  I would also be open to supporting additional minimalist windows managers other than dwm if there is succinct interest.

However, I plan to avoid feature creep: dlarm is _not_ a general purpose calendar system and will likely not support arbitrary future alarms (e.g., an alarm set for Jan. 30 of next year).  The intent is to have a targeted short-term alarms that are important enough to catch your attention—not to contribute to yet more digital noise.


## Bug reports and contributing 

dlarm is under active development and would welcome additional contributors.   Please feel free to file issues, make pull requests, or otherwise contribute.  Also feel free to submit bug reports, patches, feature requests, or comments via email to daniel@codesections.com.
