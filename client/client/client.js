
window.myr.client = (function () {

    function createClient(callbacks) {

        var connection = createConnection();

        jQuery(window).on('beforeunload', function () {
            connection.close();
        });

        function createConnection() {

            var connection = new WebSocket('ws://127.0.0.1:1337');

            connection.onopen = function () {
                onConnectionOpen();
            };

            connection.onerror = function () {
                onConnectionError();
            };

            connection.onmessage = function (event) {
                onConnectionMessage(event);
            };

            connection.onclose = function () {
                onConnectionClose();
            };

            return connection;
        };

        function onConnectionOpen() {

            console.log('Connection open');

            if (callbacks.onConnectionOpen !== undefined) {
                callbacks.onConnectionOpen();
            }
        }

        function onConnectionError() {

            console.log('Connection error');

            if (callbacks.onConnectionError !== undefined) {
                callbacks.onConnectionError();
            }
        }

        function onConnectionMessage(event) {

            console.log('Connection message');

            var json = JSON.parse(event.data);
            console.log(json);

            if (callbacks.onConnectionMessage !== undefined) {
                callbacks.onConnectionMessage(json);
            }
        }

        function onConnectionClose() {

            console.log('Connection closed');

            if (callbacks.onConnectionClose !== undefined) {
                callbacks.onConnectionClose();
            }
        }

        function sendMessage(data) {
            connection.send(data);
        };

        return {
            'sendMessage' : sendMessage
        };

    };

    var client;

    function start(callbacks) {

        if (client === undefined) {
            client = createClient(callbacks);
        } else {
            throw 'Client already started';
        }
    };

    function sendMessage(message) {

        if (client === undefined) {
            throw 'No client';
        } else {
            client.sendMessage(message);
        }
    };

    return {
        'start' : start,
        'sendMessage' : sendMessage
    };

})();
