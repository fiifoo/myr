
window.myr.index = (function () {

    var client;
    var game;

    var connection;

    function setDependencies(myr) {

        myr.game.setDependencies(myr.screen);

        client = myr.client;
        game = myr.game;
    }

    function initialize() {

        var keyboardElement = '#keyboard-commands';
        var screenElement = '#screen';

        setFocusCapture(keyboardElement);
        jQuery(keyboardElement).keydown(onKeydown);
        jQuery(screenElement).click('td', onMouseClick);

        client.start({
            'onConnectionMessage' : onMessage
        });
    };

    function setFocusCapture(keyboardElement) {

        function captureFocus() {
            jQuery(keyboardElement).focus();
        };

        jQuery(window).focus(captureFocus);
        jQuery(document).click(captureFocus);
        captureFocus();
    };

    function onKeydown(event) {

        var input = event.which;
        var action = game.getActionFromKey(input);

        if (action) {
            event.preventDefault();
            client.sendMessage(JSON.stringify({'action': action}));
        }
    };

    function onMouseClick(event) {

        var tile = jQuery(event.target).data('tile');
        var action = game.getActionFromTile(tile);

        if (action) {
            client.sendMessage(JSON.stringify({'action': action}));
        }
    };

    function onMessage(json) {

        if (json.entities !== undefined) {
            game.draw(json.entities);
        }
    }

    return {
        'setDependencies' : setDependencies,
        'initialize' : initialize
    };

})();
