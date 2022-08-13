# Renrs lang
This crate implements the renrs language, used in the game engine Renrs which is also in developemnt

## Example
```
c = Character "Crab", "./sprites/crab"
c_idle = Animation {
    c show left
    wait 1s
    c show right
}
c_idle run
c "What a fine day"
```
