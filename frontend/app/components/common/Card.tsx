import { type HTMLAttributes } from "react";
import { twMerge } from "tailwind-merge";

interface CardProps extends HTMLAttributes<HTMLDivElement> {
  title?: string;
  footer?: React.ReactNode;
}

export const Card = ({
  title,
  footer,
  children,
  className,
  ...props
}: CardProps) => {
  return (
    <div className={twMerge("card", className)} {...props}>
      {title && (
        <div className="card-header">
          <h3 className="text-lg font-medium leading-6 text-gray-900">
            {title}
          </h3>
        </div>
      )}
      <div className="card-body">{children}</div>
      {footer && <div className="card-footer">{footer}</div>}
    </div>
  );
};

interface CardHeaderProps extends HTMLAttributes<HTMLDivElement> {}

export const CardHeader = ({
  children,
  className,
  ...props
}: CardHeaderProps) => {
  return (
    <div className={twMerge("card-header", className)} {...props}>
      {children}
    </div>
  );
};

interface CardBodyProps extends HTMLAttributes<HTMLDivElement> {}

export const CardBody = ({ children, className, ...props }: CardBodyProps) => {
  return (
    <div className={twMerge("card-body", className)} {...props}>
      {children}
    </div>
  );
};

interface CardFooterProps extends HTMLAttributes<HTMLDivElement> {}

export const CardFooter = ({
  children,
  className,
  ...props
}: CardFooterProps) => {
  return (
    <div className={twMerge("card-footer", className)} {...props}>
      {children}
    </div>
  );
};
