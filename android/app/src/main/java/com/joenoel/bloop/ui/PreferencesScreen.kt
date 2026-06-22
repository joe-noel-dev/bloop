package com.joenoel.bloop.ui

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.navigationBarsPadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material3.Button
import androidx.compose.material3.DropdownMenuItem
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.ExposedDropdownMenuBox
import androidx.compose.material3.ExposedDropdownMenuDefaults
import androidx.compose.material3.HorizontalDivider
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Switch
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import bloop.Bloop
import bloop.getRequest
import bloop.request
import bloop.switchMapping
import bloop.updateRequest
import com.joenoel.bloop.state.AppAction
import com.joenoel.bloop.state.AppState

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun PreferencesScreen(
    state: AppState,
    onDispatch: (AppAction) -> Unit,
    onDismiss: () -> Unit,
) {
    var edited by remember(state.preferences) {
        mutableStateOf(state.preferences ?: Bloop.Preferences.getDefaultInstance())
    }
    var saving by remember { mutableStateOf(false) }
    var saved by remember { mutableStateOf(false) }

    LaunchedEffect(Unit) {
        onDispatch(
            AppAction.SendRequest(
                request { get = getRequest { entity = Bloop.Entity.PREFERENCES } }
            )
        )
    }

    LaunchedEffect(state.preferences) {
        if (saving && state.preferences != null) {
            saving = false
            saved = true
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .navigationBarsPadding()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = 16.dp, vertical = 8.dp),
    ) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier.fillMaxWidth().padding(bottom = 8.dp),
        ) {
            TextButton(onClick = onDismiss) { Text("Cancel") }
            Spacer(Modifier.weight(1f))
            Text(
                text = "Settings",
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.SemiBold,
            )
            Spacer(Modifier.weight(1f))
            Button(
                onClick = {
                    saving = true
                    saved = false
                    onDispatch(
                        AppAction.SendRequest(
                            request { update = updateRequest { preferences = edited } }
                        )
                    )
                },
                enabled = !saving,
            ) {
                Text(if (saved) "Saved" else "Save")
            }
        }

        HorizontalDivider()

        SectionHeader("Audio")

        OutlinedTextField(
            value = edited.audio.outputDevice,
            onValueChange = { edited = edited.toBuilder().setAudio(edited.audio.toBuilder().setOutputDevice(it)).build() },
            label = { Text("Output Device") },
            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
            singleLine = true,
        )

        NumberField(
            label = "Sample Rate",
            value = edited.audio.sampleRate.toLong(),
            onValueChange = { edited = edited.toBuilder().setAudio(edited.audio.toBuilder().setSampleRate(it.toInt())).build() },
        )

        NumberField(
            label = "Buffer Size",
            value = edited.audio.bufferSize.toLong(),
            onValueChange = { edited = edited.toBuilder().setAudio(edited.audio.toBuilder().setBufferSize(it.toInt())).build() },
        )

        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier.fillMaxWidth().padding(vertical = 8.dp),
        ) {
            Text("Use Jack", modifier = Modifier.weight(1f))
            Switch(
                checked = edited.audio.useJack,
                onCheckedChange = { edited = edited.toBuilder().setAudio(edited.audio.toBuilder().setUseJack(it)).build() },
            )
        }

        NumberField(
            label = "Main Channel Offset",
            value = edited.audio.mainChannelOffset.toLong(),
            onValueChange = { edited = edited.toBuilder().setAudio(edited.audio.toBuilder().setMainChannelOffset(it.toInt())).build() },
        )

        NumberField(
            label = "Click Channel Offset",
            value = edited.audio.clickChannelOffset.toLong(),
            onValueChange = { edited = edited.toBuilder().setAudio(edited.audio.toBuilder().setClickChannelOffset(it.toInt())).build() },
        )

        SectionHeader("MIDI")

        OutlinedTextField(
            value = edited.midi.inputDevice,
            onValueChange = { edited = edited.toBuilder().setMidi(edited.midi.toBuilder().setInputDevice(it)).build() },
            label = { Text("Input Device") },
            modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
            singleLine = true,
        )

        if (edited.switchAvailable) {
            SectionHeader("Switches")

            edited.switch.mappingsList.forEachIndexed { index, mapping ->
                SwitchMappingRow(
                    mapping = mapping,
                    onUpdate = { updated ->
                        val mappings = edited.switch.mappingsList.toMutableList().also { it[index] = updated }
                        edited = edited.toBuilder()
                            .setSwitch(edited.switch.toBuilder().clearMappings().addAllMappings(mappings))
                            .build()
                    },
                    onDelete = {
                        val mappings = edited.switch.mappingsList.toMutableList().also { it.removeAt(index) }
                        edited = edited.toBuilder()
                            .setSwitch(edited.switch.toBuilder().clearMappings().addAllMappings(mappings))
                            .build()
                    },
                )
                HorizontalDivider(modifier = Modifier.padding(vertical = 4.dp))
            }

            TextButton(
                onClick = {
                    val newMapping = switchMapping {
                        pin = 0
                        gesture = Bloop.Gesture.GESTURE_PRESS
                        action = Bloop.Action.ACTION_TOGGLE_PLAY
                    }
                    val mappings = edited.switch.mappingsList + newMapping
                    edited = edited.toBuilder()
                        .setSwitch(edited.switch.toBuilder().clearMappings().addAllMappings(mappings))
                        .build()
                },
                modifier = Modifier.padding(vertical = 4.dp),
            ) {
                Icon(Icons.Filled.Add, contentDescription = null, modifier = Modifier.padding(end = 4.dp))
                Text("Add Mapping")
            }
        }

        Spacer(Modifier.padding(bottom = 16.dp))
    }
}

@Composable
private fun SectionHeader(title: String) {
    Text(
        text = title,
        style = MaterialTheme.typography.titleMedium,
        color = MaterialTheme.colorScheme.primary,
        modifier = Modifier.padding(top = 16.dp, bottom = 4.dp),
    )
    HorizontalDivider()
}

@Composable
private fun NumberField(
    label: String,
    value: Long,
    onValueChange: (Long) -> Unit,
) {
    var text by remember(value) { mutableStateOf(value.toString()) }

    OutlinedTextField(
        value = text,
        onValueChange = { input ->
            text = input
            input.toLongOrNull()?.let(onValueChange)
        },
        label = { Text(label) },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
        singleLine = true,
    )
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun SwitchMappingRow(
    mapping: Bloop.SwitchMapping,
    onUpdate: (Bloop.SwitchMapping) -> Unit,
    onDelete: () -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp)) {
        Row(verticalAlignment = Alignment.CenterVertically, modifier = Modifier.fillMaxWidth()) {
            Text("Mapping", modifier = Modifier.weight(1f), fontWeight = FontWeight.Medium)
            IconButton(onClick = onDelete) {
                Icon(Icons.Filled.Delete, contentDescription = "Delete mapping", tint = MaterialTheme.colorScheme.error)
            }
        }

        NumberField(
            label = "Pin",
            value = mapping.pin.toLong(),
            onValueChange = { onUpdate(mapping.toBuilder().setPin(it.toInt()).build()) },
        )

        GestureDropdown(
            value = mapping.gesture,
            onSelect = { onUpdate(mapping.toBuilder().setGesture(it).build()) },
        )

        ActionDropdown(
            value = mapping.action,
            onSelect = { onUpdate(mapping.toBuilder().setAction(it).build()) },
        )
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun GestureDropdown(
    value: Bloop.Gesture,
    onSelect: (Bloop.Gesture) -> Unit,
) {
    val gestures = listOf(
        Bloop.Gesture.GESTURE_PRESS to "Press",
        Bloop.Gesture.GESTURE_RELEASE to "Release",
        Bloop.Gesture.GESTURE_HOLD to "Hold",
    )
    var expanded by remember { mutableStateOf(false) }

    ExposedDropdownMenuBox(
        expanded = expanded,
        onExpandedChange = { expanded = it },
        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
    ) {
        OutlinedTextField(
            value = gestures.firstOrNull { it.first == value }?.second ?: "Unknown",
            onValueChange = {},
            readOnly = true,
            label = { Text("Gesture") },
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded) },
            modifier = Modifier.menuAnchor().fillMaxWidth(),
        )
        ExposedDropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            gestures.forEach { (gesture, label) ->
                DropdownMenuItem(
                    text = { Text(label) },
                    onClick = {
                        onSelect(gesture)
                        expanded = false
                    },
                )
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun ActionDropdown(
    value: Bloop.Action,
    onSelect: (Bloop.Action) -> Unit,
) {
    val actions = listOf(
        Bloop.Action.ACTION_PREVIOUS_SONG to "Previous Song",
        Bloop.Action.ACTION_NEXT_SONG to "Next Song",
        Bloop.Action.ACTION_PREVIOUS_SECTION to "Previous Section",
        Bloop.Action.ACTION_NEXT_SECTION to "Next Section",
        Bloop.Action.ACTION_QUEUE_SELECTED to "Queue Selected",
        Bloop.Action.ACTION_TOGGLE_LOOP to "Toggle Loop",
        Bloop.Action.ACTION_TOGGLE_PLAY to "Toggle Play",
    )
    var expanded by remember { mutableStateOf(false) }

    ExposedDropdownMenuBox(
        expanded = expanded,
        onExpandedChange = { expanded = it },
        modifier = Modifier.fillMaxWidth().padding(vertical = 4.dp),
    ) {
        OutlinedTextField(
            value = actions.firstOrNull { it.first == value }?.second ?: "Unknown",
            onValueChange = {},
            readOnly = true,
            label = { Text("Action") },
            trailingIcon = { ExposedDropdownMenuDefaults.TrailingIcon(expanded) },
            modifier = Modifier.menuAnchor().fillMaxWidth(),
        )
        ExposedDropdownMenu(expanded = expanded, onDismissRequest = { expanded = false }) {
            actions.forEach { (action, label) ->
                DropdownMenuItem(
                    text = { Text(label) },
                    onClick = {
                        onSelect(action)
                        expanded = false
                    },
                )
            }
        }
    }
}
