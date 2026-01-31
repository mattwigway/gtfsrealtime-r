# can read alerts

    Code
      head(alerts)
    Output
                                     id      start        end agency_id route_id
      1 MTA NYCT_lmm:planned_work:22245 1735880435         NA  MTA NYCT      X38
      2 MTA NYCT_lmm:planned_work:22245 1735880435         NA  MTA NYCT      X28
      3 MTA NYCT_lmm:planned_work:29838 1768971600 1782792000  MTA NYCT     <NA>
      4 MTA NYCT_lmm:planned_work:29838 1768971600 1782792000  MTA NYCT     <NA>
      5       MTA NYCT_lmm:alert:503501 1769576637         NA  MTA NYCT     <NA>
      6       MTA NYCT_lmm:alert:503501 1769576637         NA  MTA NYCT     <NA>
        route_type direction_id trip_trip_id trip_route_id trip_direction_id
      1         NA           NA         <NA>          <NA>                NA
      2         NA           NA         <NA>          <NA>                NA
      3         NA           NA         <NA>          M15+                 0
      4         NA           NA         <NA>          M15+                 1
      5         NA           NA         <NA>           S66                 1
      6         NA           NA         <NA>           S66                 0
        trip_start_time trip_start_date trip_schedule_relationship
      1            <NA>            <NA>                       <NA>
      2            <NA>            <NA>                       <NA>
      3            <NA>            <NA>                       <NA>
      4            <NA>            <NA>                       <NA>
      5            <NA>            <NA>                       <NA>
      6            <NA>            <NA>                       <NA>
        trip_modification_id stop_id cause language cause_detail effect_detail  url
      1                 <NA>    <NA>  <NA>       EN         <NA>          <NA> <NA>
      2                 <NA>    <NA>  <NA>       EN         <NA>          <NA> <NA>
      3                 <NA>    <NA>  <NA>       EN         <NA>          <NA> <NA>
      4                 <NA>    <NA>  <NA>       EN         <NA>          <NA> <NA>
      5                 <NA>    <NA>  <NA>       EN         <NA>          <NA> <NA>
      6                 <NA>    <NA>  <NA>       EN         <NA>          <NA> <NA>
                                                                                                                                 header_text
      1                                                           X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
      2                                                           X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
      3 Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St
      4 Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St
      5                                                 S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.
      6                                                 S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.
                                                                                                                                                                                                                                                                                                                                                                                                       description_text
      1                                                              X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary stop on Surf Ave at W 22nd St.\nFor southbound service, use the stop on W 17th St at Mermaid Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
      2                                                              X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary stop on Surf Ave at W 22nd St.\nFor southbound service, use the stop on W 17th St at Mermaid Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
      3 Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St\nSee a map of this stop change.\n\n~If paying with MetroCard, obtain tickets from the vending machine at the original stop on Water St/Pine St.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
      4 Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St\nSee a map of this stop change.\n\n~If paying with MetroCard, obtain tickets from the vending machine at the original stop on Water St/Pine St.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
      5                                                   S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.\nS66 buses in both directions will not serve stops from Clove Rd/Niagara St to Arlo Rd/Stratford Ave.\n\nWhile detoured, S66 buses will make requested stops along Clove Rd.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
      6                                                   S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.\nS66 buses in both directions will not serve stops from Clove Rd/Niagara St to Arlo Rd/Stratford Ave.\n\nWhile detoured, S66 buses will make requested stops along Clove Rd.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
        tts_header_text tts_description_text severity_level
      1            <NA>                 <NA>           <NA>
      2            <NA>                 <NA>           <NA>
      3            <NA>                 <NA>           <NA>
      4            <NA>                 <NA>           <NA>
      5            <NA>                 <NA>           <NA>
      6            <NA>                 <NA>           <NA>

