
window.myr.game = (function () {

    var screen;

    function setDependencies(_screen) {
        screen = _screen;
    }

    function getActionFromKey(keycode) {
        if (keycode === 84) { // t for test
            return {
                'type': 'Move',
                'resolver': {'tile': {'x': 20, 'y': 10}}
            };
        }

        return null;
    }

    function getActionFromTile(tile) {
        return {
            'type': 'Move',
            'resolver': {'tile': tile}
        };
    }

    function draw(view) {
        screen.draw(view);
    }

    return {
        'setDependencies' : setDependencies,
        'getActionFromKey' : getActionFromKey,
        'getActionFromTile' : getActionFromTile,
        'draw' : draw
    };

})();
