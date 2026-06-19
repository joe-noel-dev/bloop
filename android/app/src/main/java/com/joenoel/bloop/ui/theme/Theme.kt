package com.joenoel.bloop.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.foundation.isSystemInDarkTheme

private val LightColors = lightColorScheme(
    primary = Ember,
    secondary = Moss,
    tertiary = Sand,
    background = Mist,
    surface = Mist,
    onPrimary = Mist,
    onSecondary = Mist,
    onBackground = Ink,
    onSurface = Ink
)

private val DarkColors = darkColorScheme(
    primary = Sand,
    secondary = Ember,
    tertiary = Moss,
    background = Ink,
    surface = Ink,
    onPrimary = Ink,
    onSecondary = Mist,
    onBackground = Mist,
    onSurface = Mist
)

@Composable
fun BloopTheme(content: @Composable () -> Unit) {
    val colorScheme = if (isSystemInDarkTheme()) DarkColors else LightColors

    MaterialTheme(
        colorScheme = colorScheme,
        typography = BloopTypography,
        content = content
    )
}

