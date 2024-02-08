import 'dart:ui';

import 'package:blabla/run.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

// Future<void> main() async {
void main() async {
  await Run.init();
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return GetMaterialApp(
      title: 'Blabla',
      enableLog: true,
      debugShowCheckedModeBanner: false,
      translations: Messages(),
      fallbackLocale: Locale('en', 'US'),
      initialRoute: AppPages.INITIAL,
      getPages: AppPages.routes,
      unknownRoute: AppPages.unknownRoute,
      locale: TranslationService.locale,
      defaultTransition: Transition.rightToLeft,
      builder: EasyLoading.init(),
      theme: ThemeData().copyWith(
        brightness: Brightness.light,
        primaryColor: Color.app_main,
      ),
    );
  }
}
