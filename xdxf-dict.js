$(document).ready(function() {
    $('#search').click(function() {
        $.getJSON('', {req: $('#req').val()}, function(data) {
            var items = [];
            $.each(data, function(k, v) {
                items.push('<h4>'+k+'</h4><p>'+v+'</p>');
            });
            $('#searchresults').html($('<div/>', {
                html: items.join('')
            }));
        });
    });
    $('#req').keypress(function(event) {
        if (event.which == 13) {
            event.preventDefault();
            $('#search').click();
        }
    });
    $('#req').autocomplete({
        url: '',
        showResult: function(k) {
            return '<span style="color:red">' + k + '</span>';
        },
        processData: function(data) {
            var i, j = 0, processed = [];
            for (i in data) {
                j++;
                if(j>5) break;
                processed.push(i);
            }
            return processed;
        },
        onItemSelect: function(item) {
            var text = item.value;
            $('#req').val(text);
            $('#search').click();
        },
        minChars: 3,
        maxItemsToShow: 0,
        selectOnly: true,
        useCache: false,
        queryParamName: 'req',
        remoteDataType: 'json'
        });
});
