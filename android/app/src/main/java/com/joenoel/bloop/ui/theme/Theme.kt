package com.joenoel.bloop.ui.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.foundation.isSystemInDarkTheme

private val LightColors = lightColorScheme(
    primary = BloopTheme1,
    onPrimary = BloopNeutral7,
    secondary = BloopTheme3,
    onSecondary = BloopNeutral0,
    tertiary = BloopTheme2,
    onTertiary = BloopNeutral7,
    error = BloopTheme4,
    onError = BloopNeutral0,
    background = BloopBackgroundLight,
    onBackground = BloopNeutral7,
    surface = BloopNeutral1,
    onSurface = BloopNeutral7,
    surfaceVariant = BloopNeutral1,
    onSurfaceVariant = BloopNeutral5,
)

private val DarkColors = darkColorScheme(
    primary = BloopTheme1,
    onPrimary = BloopNeutral7,
    secondary = BloopTheme3,
    onSecondary = BloopNeutral0,
    tertiary = BloopTheme2,
    onTertiary = BloopNeutral7,
    error = BloopTheme4,
    onError = BloopNeutral0,
    background = BloopBackgroundDark,
    onBackground = BloopNeutral0,
    surface = BloopNeutral7,
    onSurface = BloopNeutral1,
    surfaceVariant = BloopNeutral6,
    onSurfaceVariant = BloopNeutral2,
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

