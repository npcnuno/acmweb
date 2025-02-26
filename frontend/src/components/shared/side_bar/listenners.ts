
export function on_resize( event?: Event, reset?: Boolean ) {

  // Query Selectors for css variables manipulation
  const side_bar_variables = document.querySelector< HTMLElement >('#side-bar') ?? null;
  const layout_variables = document.querySelector< HTMLElement >('#full-page') ?? null;

  // Check if successfully request the selectors
  if( side_bar_variables === null || layout_variables === null ) return;
 
  // Values for the new resized screen size
  var side_bar_width = '330px';
  var side_bar_display_flex = 'flex';
  var side_bar_normal_screen_display = 'block';
  var side_bar_normal_screen_display_flex = 'flex';
  var side_bar_small_screen_display = 'none';
  var side_bar_phone_screen_display_flex = 'none';

  var side_bar_user_photo_size = '140px';
  var side_bar_user_background_height = '250px';

  var side_bar_button_icon_margin = '20px';

  var side_bar_footer_notifications_count_left = '245px';
  var side_bar_footer_notifications_popup_margin_left = '0px';

  var layout_top_padding_slot = '20px';
  var layout_left_padding_slot = '350px';

  // Depending on the screen size resize all the html/css to the correct values
  
  if( window.innerWidth < 700 && reset === undefined ) {

    side_bar_width = '0px';
    side_bar_normal_screen_display = 'none';
    side_bar_display_flex = 'none';
    side_bar_normal_screen_display_flex = 'none';
    side_bar_small_screen_display = 'none';
    side_bar_phone_screen_display_flex = 'flex';

    side_bar_user_background_height = '0px';
    side_bar_user_photo_size = '0px';

    side_bar_footer_notifications_count_left  = '-30px';
    side_bar_footer_notifications_popup_margin_left  = '0px';

    layout_top_padding_slot = '70px';
    layout_left_padding_slot = '20px';

  }

  else if( window.innerWidth < 1300 && reset === undefined ) {
  
    side_bar_width = '80px';
    side_bar_normal_screen_display = 'none';
    side_bar_normal_screen_display_flex = 'none';
    side_bar_small_screen_display = 'block';

    side_bar_user_background_height = '150px';
    side_bar_user_photo_size = '60px';

    side_bar_button_icon_margin = 'auto';

    side_bar_footer_notifications_count_left = '45px';
    side_bar_footer_notifications_popup_margin_left  = '220px';

    layout_left_padding_slot = '100px';
    
  }

  // Change the css values 
  
  side_bar_variables.style.setProperty( '--side-bar-width', side_bar_width );
  side_bar_variables.style.setProperty( '--side-bar-display-flex', side_bar_display_flex );
  side_bar_variables.style.setProperty( '--side-bar-phone-screen-display-flex', side_bar_phone_screen_display_flex );
  side_bar_variables.style.setProperty( '--side-bar-normal-screen-display', side_bar_normal_screen_display );
  side_bar_variables.style.setProperty( '--side-bar-normal-screen-display-flex', side_bar_normal_screen_display_flex );
  side_bar_variables.style.setProperty( '--side-bar-small-screen-display', side_bar_small_screen_display );
  
  side_bar_variables.style.setProperty( '--side-bar-user-photo-background-height', side_bar_user_background_height );
  side_bar_variables.style.setProperty( '--side-bar-user-photo-size', side_bar_user_photo_size );
  
  side_bar_variables.style.setProperty( '--side-bar-button-icon-margin', side_bar_button_icon_margin );

  side_bar_variables.style.setProperty( '--side-bar-footer-notifications-count-left', side_bar_footer_notifications_count_left );
  side_bar_variables.style.setProperty( '--side-bar-footer-notifications-popup-margin-left', side_bar_footer_notifications_popup_margin_left );

  if( reset === undefined ) {
    
    layout_variables.style.setProperty( '--layout-top-padding-slot', layout_top_padding_slot );
    layout_variables.style.setProperty( '--layout-left-padding-slot', layout_left_padding_slot );
  
  }

}

 
