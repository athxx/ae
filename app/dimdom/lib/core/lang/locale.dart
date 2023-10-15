import 'package:dimdom/core/lang/en.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';

class LocaleService extends Translations {
  static Locale? get locale => Get.deviceLocale;
  static const fallbackLocale = Locale('en', 'US');
  @override
  Map<String, Map<String, String>> get keys => {'en': en_US};
}
