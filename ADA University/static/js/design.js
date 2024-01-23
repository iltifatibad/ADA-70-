$('#solar-system').change(function () {                    
    var fontsize = $(this).val()/10 + "px";
    $('html').css("font-size", fontsize );
  });