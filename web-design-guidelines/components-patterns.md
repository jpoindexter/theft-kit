# Component Patterns

## Layout Components

### Container

Consistent max-width and padding:

```jsx
const Container = ({ children, size = 'default', className }) => {
  const sizes = {
    narrow: 'max-w-3xl',
    default: 'max-w-5xl', 
    wide: 'max-w-7xl',
    full: 'max-w-none'
  };
  
  return (
    <div className={`${sizes[size]} mx-auto px-4 sm:px-6 lg:px-8 ${className}`}>
      {children}
    </div>
  );
};
```

### Grid System

Flexible grid with consistent gaps:

```jsx
const Grid = ({ children, cols = 3, gap = 6 }) => {
  const colClasses = {
    1: 'grid-cols-1',
    2: 'grid-cols-1 md:grid-cols-2',
    3: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-3',
    4: 'grid-cols-1 md:grid-cols-2 lg:grid-cols-4'
  };
  
  return (
    <div className={`grid ${colClasses[cols]} gap-${gap}`}>
      {children}
    </div>
  );
};
```

### Stack

Vertical spacing with consistent rhythm:

```jsx
const Stack = ({ children, spacing = 4, className }) => (
  <div className={`flex flex-col space-y-${spacing} ${className}`}>
    {children}
  </div>
);
```

## Interactive Components

### Button Variants

```jsx
const Button = ({ 
  children, 
  variant = 'primary', 
  size = 'md',
  isLoading = false,
  ...props 
}) => {
  const baseStyles = 'inline-flex items-center justify-center font-medium rounded-lg transition-colors focus:outline-none focus:ring-2 focus:ring-offset-2';
  
  const variants = {
    primary: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500',
    secondary: 'bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-500',
    outline: 'border-2 border-blue-600 text-blue-600 hover:bg-blue-50 focus:ring-blue-500',
    ghost: 'text-blue-600 hover:bg-blue-50 focus:ring-blue-500'
  };
  
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  };
  
  return (
    <button 
      className={`${baseStyles} ${variants[variant]} ${sizes[size]}`}
      disabled={isLoading}
      {...props}
    >
      {isLoading ? <Spinner className="mr-2" /> : null}
      {children}
    </button>
  );
};
```

### Card Component

```jsx
const Card = ({ children, className, hover = false }) => (
  <div className={`
    bg-white rounded-lg shadow-md overflow-hidden
    ${hover ? 'transition-shadow hover:shadow-lg' : ''}
    ${className}
  `}>
    {children}
  </div>
);

const CardHeader = ({ title, subtitle, action }) => (
  <div className="px-6 py-4 border-b border-gray-200 flex justify-between items-center">
    <div>
      <h3 className="text-lg font-semibold text-gray-900">{title}</h3>
      {subtitle && <p className="text-sm text-gray-500">{subtitle}</p>}
    </div>
    {action && <div>{action}</div>}
  </div>
);

const CardBody = ({ children, className }) => (
  <div className={`px-6 py-4 ${className}`}>{children}</div>
);

const CardFooter = ({ children, className }) => (
  <div className={`px-6 py-4 bg-gray-50 border-t border-gray-200 ${className}`}>
    {children}
  </div>
);
```

### Modal/Dialog

```jsx
const Modal = ({ isOpen, onClose, title, children, footer }) => {
  if (!isOpen) return null;
  
  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex items-center justify-center min-h-screen px-4">
        <div 
          className="fixed inset-0 bg-black bg-opacity-50 transition-opacity"
          onClick={onClose}
        />
        <div className="relative bg-white rounded-lg shadow-xl max-w-lg w-full">
          <div className="flex justify-between items-center p-6 border-b">
            <h2 className="text-xl font-semibold">{title}</h2>
            <button onClick={onClose} className="text-gray-400 hover:text-gray-600">
              <XIcon className="w-6 h-6" />
            </button>
          </div>
          <div className="p-6">{children}</div>
          {footer && <div className="p-6 border-t bg-gray-50">{footer}</div>}
        </div>
      </div>
    </div>
  );
};
```

## Form Patterns

### Input with Label

```jsx
const InputGroup = ({ label, error, helperText, children }) => (
  <div className="space-y-1">
    <label className="block text-sm font-medium text-gray-700">
      {label}
    </label>
    {children}
    {error && <p className="text-sm text-red-600">{error}</p>}
    {helperText && !error && <p className="text-sm text-gray-500">{helperText}</p>}
  </div>
);

const TextInput = ({ label, error, helperText, ...props }) => (
  <InputGroup label={label} error={error} helperText={helperText}>
    <input
      className={`
        w-full px-3 py-2 border rounded-lg shadow-sm
        focus:outline-none focus:ring-2 focus:ring-blue-500
        ${error ? 'border-red-500' : 'border-gray-300'}
      `}
      {...props}
    />
  </InputGroup>
);
```

### Form Layout

```jsx
const Form = ({ children, onSubmit, className }) => (
  <form onSubmit={onSubmit} className={`space-y-6 ${className}`}>
    {children}
  </form>
);

const FormSection = ({ title, description, children }) => (
  <div className="space-y-4">
    <div>
      <h3 className="text-lg font-medium text-gray-900">{title}</h3>
      {description && <p className="text-sm text-gray-500">{description}</p>}
    </div>
    <div className="space-y-4">
      {children}
    </div>
  </div>
);

const FormActions = ({ children, className }) => (
  <div className={`flex items-center justify-end space-x-4 pt-6 border-t ${className}`}>
    {children}
  </div>
);
```
