## rust system service frame template 
## support windows linux macos


* service cmd
``` sh
  test_service service install
  test_service service install config.json
  test_service service install /home/user/xxxx.config.json
  test_service service start
  test_service service stop
  test_service service restart
  test_service service uninstall
```

* exec service
``` sh
   test_service exec 
   test_service exec config.json
   test_service exec /home/user/xxxx.config.json
```

* create config file to path
``` sh
  test_service create
  test_service create d:/xxx.json
  test_service create /home/user/xxxx.config.json
```

* usage examples
``` sh
  test_service create
  test_service service install
  test_service service start
```

* logs out  
  log output directory "logs" to service exec file path 
