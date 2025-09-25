import {Middleware} from './middleware';

export const loggingMiddleware: Middleware =
  (_api) => (next) => async (action) => {
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
      throw error;
    }
  };
