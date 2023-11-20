import 'dart:io';

import 'package:dimdom/core/services/services.dart';
import 'package:dimdom/core/store/store.dart';
import 'package:dimdom/core/utils/utils.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:get/get.dart';
import 'package:hive_flutter/hive_flutter.dart';

/// Global initialize
class Run {
  static GetxController? currentPage;
  static Future init() async {
    WidgetsFlutterBinding.ensureInitialized();

    /// only accept up portraitUp
    // don't use await
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.portraitUp,
      // DeviceOrientation.portraitDown,
      // DeviceOrientation.landscapeLeft,
      // DeviceOrientation.landscapeRight,
    ]);

    /// Hive initial
    Hive.initFlutter(); // don't use await

    /// SharedPreferences
    // await SharedPreferences.getInstance();

    /// Dio initialize
    // Dio();

    /// TODO Map Initialize
    // OSM_init();

    /// TODO whether is first open

    setSystemUi();

    // await Get.putAsync(() => SharedPreferences.getInstance());
    // Get.lazyPut(() =>RequestRepository());
    Loading();

    await Get.putAsync<StorageService>(() => StorageService().init());

    Get.put<ConfigStore>(ConfigStore());
    Get.put<UserStore>(UserStore());
  }

  static void setSystemUi() {
    // TODO maybe remove one judge condition
    if (Platform.isAndroid || GetPlatform.isAndroid) {
      SystemUiOverlayStyle systemUiOverlayStyle = const SystemUiOverlayStyle(
        statusBarColor: Colors.transparent,
        statusBarBrightness: Brightness.light,
        statusBarIconBrightness: Brightness.dark,
        systemNavigationBarDividerColor: Colors.transparent,
        systemNavigationBarColor: Colors.white,
        systemNavigationBarIconBrightness: Brightness.dark,
      );
      SystemChrome.setSystemUIOverlayStyle(systemUiOverlayStyle);
    }

    // 适配高帧率
    GestureBinding.instance.resamplingEnabled = true;
  }
}
