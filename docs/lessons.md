### Emulating a Tabbed-Panel and Screen-Manager

I wanted something like Kivy's Tabbed-Panel
 - https://kivy.org/doc/stable/api-kivy.uix.tabbedpanel.html

and a Screen-manager to handle it. Using Egui, I've managed to have
Buttons that kinda-sort act as a Tabbed-Panel. I really need to understand
the library a lot better - that, or find another GUI-library. As for a
Screen-Manager, I'm using custom States for that. Not using Bevy States,
as they are likely to change in the next release - in fact, cheatbook
recommends another library, `iyes-loopless`, but not interested in adding
another library for what is fairly simple to do manually.

So States are just an enum, and all the Systems related to a particular
State will have code like:

    if screen_mgr.current_screen != CurScreen::ActionScreen { return; }

Currently, the only States are LoadScreen and ActionScreen.

Note: this means that all Systems are running all the time, with just a boolean-check
at the start for early-exit. I don't know of any implications for the app - if I
used the built-in States or Run-criteria, there's still be some kind of run-check, but
one less function-call. Fairly sure a few extra function-calls, won't slow down the app.

My only real concern is *readability* - I've tried to clarify which Systems are for the
Action-Screen. Even though the Load-Screen doesn't do anything, it might someday, and
there might be other Screens.

### Getting mouse/cursor-location

https://bevy-cheatbook.github.io/cookbook/cursor2world.html#2d-games

### Mapping Enum values to Strings

https://kerkour.com/rust-enum-to-string

Note that's definitely an upper-case `String` - which is not the same
thing as a `str`. However, the `asset_loader.load()` takes a `str`.
I haven't found how to make the Enum.Display do that, so have to
convert it on loading:

    let foo: &str = &*OneBackground::Map1.to_string();

Originally I had used the struct `BackgroundMap`, just to mark the Sprite that
was displaying the background-bitmap. Then I decided to use that struct, to
contain the currently-displayed bitmap. And that's when I realized that I was
*despawning* the background-Sprite, when the spacebar was hit. Which means that
my struct was going away. So had to make it a separate Resource.

This still kinda freaks me out - an empty struct?

    pub struct IsBackground;

### Using multiple mutable Components?
This definitely doesn't work, there are two Transforms, one of which is mutable:

    mut sprite_position: Query<(&mut CarDirection, &mut Transform)>,
    drag_points: Query<&HoverCraft, &Transform>,

https://github.com/bevyengine/bevy/issues/2198

I'm impressed that it's possible to make disjoint Queries, that Bevy recognizes. However, the
advice given above, suggests that having more than one Query can result in problems. I eventually
wound up refactoring things - the Draggable Sprites, basically posted their current
position as a Resource, so the moving crab-Sprite, could use that.

Of course, that required changing the crab-Sprite movement, so it moved between two other Sprites.
Although I gave those sprites a left-dir and right-dir marker - they can be moved around, so
the left Sprite *could* be on the right of the right Sprite. Therefore, the crab-Sprite itself,
needed a marker of what direction it was going. Fortunately, I already had one, sort of -
it was up/down/left/right, from a time when it could move in four direction. But it was a start.

### Cool Enum Tricks

https://stackoverflow.com/questions/25867875/how-do-i-toggle-through-enum-variants

### Gui stuff

So it turns out

    pub fn do_ui_setup(mut egui_context: ResMut<EguiContext>,
        mut random_data: ResMut<GuiData>,) {
        let age: i32 = 0; }

doesn't work in Immediate-Mode. Wups. IM means every frame,
the function refreshes the screen, and constructs the UI anew.
Any data has to be stored somewhere else, like a Resource.
