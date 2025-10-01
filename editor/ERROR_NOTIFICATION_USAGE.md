# Error Notification System

The editor now includes a general-purpose error notification system that allows developers to display error messages to users with automatic dismissal functionality.

## Features

- **Show Error Notifications**: Display error messages in a prominent notification bar at the top of the screen
- **Auto-dismiss**: Notifications automatically disappear after 5 seconds
- **Manual dismiss**: Users can manually close notifications using the close button
- **Single notification**: Only one error notification is shown at a time (new errors replace existing ones)

## Usage

### Showing an Error Notification

To display an error notification, dispatch the `showErrorNotificationAction`:

```typescript
import { useDispatcher } from '../dispatcher/dispatcher';
import { showErrorNotificationAction } from '../dispatcher/action';

const MyComponent = () => {
  const dispatch = useDispatcher();

  const handleError = () => {
    dispatch(showErrorNotificationAction('Something went wrong! Please try again.'));
  };

  // Component JSX...
};
```

### Manually Hiding an Error Notification

While notifications auto-dismiss, you can also manually hide them:

```typescript
import { hideErrorNotificationAction } from '../dispatcher/action';

const hideError = () => {
  dispatch(hideErrorNotificationAction());
};
```

## Implementation Details

### State Structure

The error notification state is stored in the main `AppState`:

```typescript
interface ErrorNotification {
  id: string;
  message: string;
  timestamp: number;
}

interface AppState {
  // ... other state properties
  errorNotification?: ErrorNotification;
}
```

### Actions

- `SHOW_ERROR_NOTIFICATION`: Creates and displays a new error notification
- `HIDE_ERROR_NOTIFICATION`: Removes the current error notification

### Component

The `ErrorNotificationBar` component handles the display and auto-dismiss logic:

- Positioned at the top-center of the screen with high z-index
- Styled as a soft danger-colored alert using MUI Joy components
- Automatically sets up a 5-second timer for dismissal
- Includes a close button for manual dismissal

## Examples

### Basic Error Notification
```typescript
dispatch(showErrorNotificationAction('Failed to save project'));
```

### Network Error
```typescript
try {
  await api.saveProject();
} catch (error) {
  dispatch(showErrorNotificationAction('Network error: Unable to save project. Please check your connection.'));
}
```

### Validation Error
```typescript
if (!projectName.trim()) {
  dispatch(showErrorNotificationAction('Project name cannot be empty'));
  return;
}
```