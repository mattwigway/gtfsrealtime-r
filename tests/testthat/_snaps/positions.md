# can read positions

    Code
      head(positions)
    Output
                   id latitude longitude   bearing odometer speed
      1 MTA NYCT_9771 40.78720 -73.95419  55.21397       NA    NA
      2 MTA NYCT_8440 40.74109 -73.78681 113.49857       NA    NA
      3 MTA NYCT_9770 40.87092 -73.84709 152.70042       NA    NA
      4 MTA NYCT_7112 40.65890 -73.89967  19.17901       NA    NA
      5 MTA NYCT_8443 40.75879 -73.82975 293.74948       NA    NA
      6 MTA NYCT_9775 40.85025 -73.93855 248.07346       NA    NA
                                    trip_id route_id direction_id start_time
      1    MV_A6-Weekday-SDon-110100_M5_527       M4            0       <NA>
      2   JA_A6-Weekday-SDon-110800_Q17_334      Q17            0       <NA>
      3 GH_A6-Weekday-SDon-111400_BX238_640     BX28            1       <NA>
      4   EN_A6-Weekday-SDon-108800_B15_142      B15            0       <NA>
      5   JA_A6-Weekday-SDon-113200_Q17_317      Q17            1       <NA>
      6    MV_A6-Weekday-SDon-113200_M4_454       M4            1       <NA>
        start_date schedule_relationship stop_id current_stop_sequence current_status
      1   20260121                  <NA>  400041                    NA           <NA>
      2   20260121                  <NA>  501341                    NA           <NA>
      3   20260121                  <NA>  101927                    NA           <NA>
      4   20260121                  <NA>  301136                    NA           <NA>
      5   20260121                  <NA>  501369                    NA           <NA>
      6   20260121                  <NA>  400645                    NA           <NA>
                  timestamp congestion_level     occupancy_status
      1 2026-01-21 18:58:29             <NA>   STANDING_ROOM_ONLY
      2 2026-01-21 18:58:24             <NA>                 <NA>
      3 2026-01-21 18:58:32             <NA> MANY_SEATS_AVAILABLE
      4 2026-01-21 18:58:05             <NA>  FEW_SEATS_AVAILABLE
      5 2026-01-21 18:58:32             <NA>                 <NA>
      6 2026-01-21 18:58:06             <NA> MANY_SEATS_AVAILABLE
        occupancy_percentage    vehicle_id vehicle_label vehicle_license_plate
      1                   NA MTA NYCT_9771          <NA>                  <NA>
      2                   NA MTA NYCT_8440          <NA>                  <NA>
      3                   NA MTA NYCT_9770          <NA>                  <NA>
      4                   NA MTA NYCT_7112          <NA>                  <NA>
      5                   NA MTA NYCT_8443          <NA>                  <NA>
      6                   NA MTA NYCT_9775          <NA>                  <NA>

