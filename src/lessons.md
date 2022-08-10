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
