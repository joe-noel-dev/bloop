import {Middleware} from './middleware';
import {showErrorNotificationAction} from './action';

export const loggingMiddleware: Middleware =
  (api) => (next) => async (action) => {
    if (process.env.NODE_ENV === 'production') {
      await next(action);
      return;
    }

    console.group(`Action: ${action.type}`);
    try {
      await next(action);
      console.groupEnd();
    } catch (error) {
      console.error('❌ Action failed:', error);
      console.groupEnd();
      
      // Show user-friendly error notification
      if (error instanceof Error) {
        api.dispatch(showErrorNotificationAction(`Action failed: ${error.message}`));
      } else {
        api.dispatch(showErrorNotificationAction('An unexpected error occurred'));
      }
      
      throw error;
    }
  };
