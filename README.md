# Renrs lang
This create implements the renrs language, used in the game engine Renrs

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
