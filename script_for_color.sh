  if [ $(date +"%s") -ge ${ALARM_TIME} ]; then
    xsetroot -name \
      "                                    ${ALARM_MESSAGE}                                       "
    sleep 1
    xsetroot -name \
      "                                    ${ALARM_MESSAGE}                                       "
    sleep 1
    else 
      # previous `xsetroot -name` command
    sleep 3
  fi
