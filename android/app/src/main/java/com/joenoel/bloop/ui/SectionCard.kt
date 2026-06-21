package com.joenoel.bloop.ui

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.IntrinsicSize
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GraphicEq
import androidx.compose.material.icons.filled.Repeat
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import bloop.Bloop

@Composable
internal fun SectionCard(
    section: Bloop.Section,
    selections: Bloop.Selections,
    playbackState: Bloop.PlaybackState,
    progress: Bloop.Progress,
    onSelect: () -> Unit,
) {
    val isSelected = selections.section == section.id
    val isPlaying = playbackState.sectionId == section.id
    val borderColor = when {
        isPlaying -> MaterialTheme.colorScheme.primary
        isSelected -> MaterialTheme.colorScheme.onSurfaceVariant
        else -> Color.Transparent
    }

    Surface(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onSelect),
        shape = RoundedCornerShape(4.dp),
        color = MaterialTheme.colorScheme.surfaceVariant,
    ) {
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .defaultMinSize(minHeight = 64.dp)
                .height(IntrinsicSize.Min),
            contentAlignment = Alignment.CenterStart,
        ) {
            if (isPlaying) {
                Box(
                    modifier = Modifier
                        .fillMaxHeight()
                        .fillMaxWidth(progress.sectionProgress.coerceIn(0.0, 1.0).toFloat())
                        .background(MaterialTheme.colorScheme.primary.copy(alpha = 0.3f)),
                )
            }

            Box(
                modifier = Modifier
                    .fillMaxHeight()
                    .width(4.dp)
                    .background(borderColor),
            )

            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 14.dp, vertical = 12.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = section.name.takeIf { it.isNotBlank() } ?: "Untitled section",
                    style = MaterialTheme.typography.headlineSmall,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onSurface,
                    modifier = Modifier.weight(1f),
                )

                Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                    if (section.loop) {
                        Icon(
                            imageVector = Icons.Filled.Repeat,
                            contentDescription = "Loop section",
                            tint = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                    if (section.metronome) {
                        Icon(
                            imageVector = Icons.Filled.GraphicEq,
                            contentDescription = "Metronome enabled",
                            tint = MaterialTheme.colorScheme.onSurfaceVariant,
                        )
                    }
                }
            }
        }
    }
}
