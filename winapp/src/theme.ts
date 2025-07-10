// Global dark theme palette inspired by Discord and MS Teams
export const darkTheme = {
  // Primary colors - Discord-like blues
  primary: '#5865F2',      // Discord primary blue
  secondary: '#4752C4',    // Discord secondary blue
  accent: '#7289DA',       // Discord accent blue
  
  // Dark backgrounds - Discord/MS Teams dark theme
  dark: '#2C2F33',         // Discord dark gray
  darkPage: '#23272A',     // Discord darker gray
  
  // Status colors
  positive: '#43B581',     // Discord green
  negative: '#F04747',     // Discord red
  info: '#00B0F4',         // MS Teams blue
  warning: '#FAA61A',      // Discord yellow
  
  // Additional theme colors
  surface: '#36393F',      // Discord surface gray
  surfaceHover: '#40444B', // Discord hover gray
  textPrimary: '#FFFFFF',  // White text
  textSecondary: '#B9BBBE', // Discord secondary text
  textMuted: '#72767D',    // Discord muted text
  border: '#202225',       // Discord border
  shadow: 'rgba(0, 0, 0, 0.3)', // Dark shadow
  
  // Overlay specific colors
  overlayBackground: 'rgba(0, 0, 0, 0.3)',
  overlaySurface: 'rgba(54, 57, 63, 0.95)',
  overlaySurfaceHover: 'rgba(64, 68, 75, 0.95)',
  overlayBorder: 'rgba(255, 255, 255, 0.1)',
  overlayBorderHover: 'rgba(255, 255, 255, 0.2)',
  
  // Monotone foreground colors for better readability
  textLight: '#E8E8E8',    // Light gray text (high contrast)
  textMedium: '#C8C8C8',   // Medium gray text (medium contrast)
  textSoft: '#A8A8A8',     // Soft gray text (low contrast)
  textSubtle: '#888888',   // Subtle gray text (very low contrast)
  
  // Button borders
  buttonBorder: 'rgba(255, 255, 255, 0.08)',      // Very subtle border
  buttonBorderHover: 'rgba(255, 255, 255, 0.15)', // Slightly more visible on hover
  buttonBorderActive: 'rgba(255, 255, 255, 0.25)', // More visible when active
} as const;

// Type for theme colors
export type ThemeColors = typeof darkTheme;

// Helper function to get theme color with opacity
export const getThemeColor = (color: keyof ThemeColors, opacity: number = 1): string => {
  const hexColor = darkTheme[color];
  if (hexColor.startsWith('#')) {
    // Convert hex to rgba
    const r = parseInt(hexColor.slice(1, 3), 16);
    const g = parseInt(hexColor.slice(3, 5), 16);
    const b = parseInt(hexColor.slice(5, 7), 16);
    return `rgba(${r}, ${g}, ${b}, ${opacity})`;
  }
  return hexColor;
};

// Export default theme
export default darkTheme; 