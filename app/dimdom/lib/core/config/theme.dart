import 'package:flutter/material.dart';

ThemeData lightTheme = ThemeData(
  brightness: Brightness.light,
  primarySwatch: Colors.blue,
  colorScheme: const ColorScheme.light(
    primary: Colors.blue,
    secondary: Colors.amber,
  ),
);

ThemeData darkTheme = ThemeData(
  brightness: Brightness.dark,
  primarySwatch: Colors.grey,
  colorScheme: const ColorScheme.dark(
    primary: Colors.grey,
    secondary: Colors.amber,
  ),
);

ThemeData appThemeData(Brightness brightness) {
  return brightness == Brightness.dark ? darkTheme : lightTheme;
}
