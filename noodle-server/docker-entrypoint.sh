#!/bin/sh
/bin/noodle-server & nginx -g 'daemon off;'

wait -n

exit $?
