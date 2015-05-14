
window.myr.screen = (function () {

    var configs = {
        sizeX: 100,
        sizeY: 30,
    };
    var element;

    function draw(entities) {

        if (element === undefined) {
            initializeElement();
        }

        clearScreen();

        jQuery.each(entities, function (index, entity) {
            var x = entity.tile.x;
            var y = entity.tile.y;

            if (element.rows[y] !== undefined && element.rows[y].cells[x] !== undefined) {
                element.rows[y].cells[x].innerHTML = entity.id;
            }
        });
    }

    function initializeElement() {

        if (!configs.sizeX) {
            throw 'Screen sizeX not available';
        }
        if (!configs.sizeY) {
            throw 'Screen sizeX not available';
        }

        var table = jQuery('<table>');
        table.attr('id', 'screen-content');

        for (var y = 0; y < configs.sizeY; y++) {
            var tr = jQuery('<tr>');
            for (var x = 0; x < configs.sizeX; x++) {
                var td = jQuery('<td>');
                td.data('tile', {'x': x, 'y': y});
                tr.append(td);
            }
            table.append(tr);
        }

        jQuery('#screen').append(table);

        element = table.get(0);
    }

    function clearScreen() {
        jQuery(element).find('td').html('');
    }

    return {
        'draw' : draw
    };

})();
