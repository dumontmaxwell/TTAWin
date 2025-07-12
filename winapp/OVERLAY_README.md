# TTAWin Overlay System

## Overview
TTAWin features a Windows-specific overlay system that provides a transparent, full-screen overlay for developer controls and monitoring.

## Hotkeys
- **Ctrl+Shift+H**: Toggle overlay visibility
- **Ctrl+Shift+M**: Toggle microphone
- **Ctrl+Shift+N**: Switch monitor
- **Ctrl+Shift+S**: Open settings
- **Ctrl+Shift+Q**: Quit application

## Overlay Features

### Window Behavior
- **Full Screen**: Covers entire screen with transparency
- **Click-Through**: When overlay is hidden, clicks pass through to underlying applications
- **Always on Top**: Stays above other windows when visible
- **Transparent Background**: Allows visibility of content behind the overlay

### Developer Controls
- **Microphone Toggle**: Start/stop audio recording
- **Monitor Switching**: Switch between multiple monitors
- **Settings Access**: Quick access to application settings
- **Quit Application**: Exit the application

### Visual Elements
- **Developer Panel**: Central status display showing:
  - Microphone status (ON/OFF)
  - Number of available monitors
  - Current monitor index
- **Control Buttons**: Top-right corner controls for quick access
- **Settings Modal**: Full-screen modal for configuration

## Technical Implementation

### Windows API Integration
The overlay uses native Windows APIs for:
- `SetLayeredWindowAttributes`: Control transparency
- `SetWindowLongPtrW`: Modify window styles for click-through
- `SetWindowPos`: Position and show/hide window
- `RegisterHotKey`: Global hotkey registration

### State Management
- **Pinia Store**: Centralized state management for overlay visibility
- **Reactive Updates**: Real-time UI updates based on state changes
- **Event System**: Hotkey events trigger overlay actions

### Performance Optimizations
- **Lazy Loading**: Commands loaded only when needed
- **Efficient Rendering**: CSS transitions for smooth animations
- **Memory Management**: Proper cleanup of event listeners

## Usage

1. **Start Application**: Launch TTAWin
2. **Toggle Overlay**: Press `Ctrl+Shift+H` to show/hide overlay
3. **Use Controls**: Click buttons or use hotkeys for actions
4. **Access Settings**: Use `Ctrl+Shift+S` or click settings button
5. **Quit**: Use `Ctrl+Shift+Q` or click quit button

## Troubleshooting

### Overlay Not Showing
- Check if hotkeys are registered properly
- Verify window transparency settings
- Ensure application has proper permissions

### Click-Through Issues
- Verify `set_click_through` command is working
- Check window styles are set correctly
- Ensure overlay is properly hidden when needed

### Performance Issues
- Monitor memory usage
- Check for event listener leaks
- Verify efficient state updates

## Development Notes

### Windows-Specific Features
- Uses Windows crate for native API calls
- Leverages Windows-specific window styles
- Optimized for Windows performance

### Cross-Platform Considerations
- Currently Windows-only implementation
- No macOS or Linux support
- Uses Windows-specific APIs throughout 