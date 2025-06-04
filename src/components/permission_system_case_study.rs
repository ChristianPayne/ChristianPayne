use leptos::prelude::*;
use thaw::*;

#[component]
pub fn PermissionSystemCaseStudy() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            // Hero Section
            <div class="relative overflow-hidden">
                <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-24">
                    <div class="text-center">
                        <h1 class="text-5xl md:text-7xl font-bold mb-2">"Permission System"</h1>
                        <p class="text-lg text-gray-600 mb-6">"Model Match"</p>
                        <p class="text-xl md:text-2xl mb-8 max-w-3xl mx-auto">
                            "A comprehensive role-based access control system implementation, featuring granular permissions, role management, and realtime updates."
                        </p>
                        <div class="flex flex-wrap justify-center gap-4 mb-8">
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "TypeScript"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "GraphQL"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "AppSync"
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            // Project Overview
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-6">
                            <h2 class="text-4xl font-bold mb-6">"Project Overview"</h2>
                            <p class="text-lg">
                                "Developed as a core component of Model Match, this permission system was created to handle the complex access control needs of a multi-tenant platform. It enables precise control over who can access what features, ensuring that each user has exactly the permissions they need while maintaining security and flexibility. All changes are applied instantly, with no need for users to refresh or restart their sessions."
                            </p>
                            <p class="text-lg">
                                "At its core, the system provides a flexible way to manage user access through roles. Every user starts with basic permissions, while administrators have full control over the platform. This structure allows Model Match to easily manage access for different types of users, from basic users to power users and administrators."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // History & Motivation
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-6">
                            <h2 class="text-4xl font-bold mb-6">"History & Motivation"</h2>
                            <p class="text-lg">
                                "Model Match was built as a multi-tenant platform from the start, but the initial permission system was limited. Each tenant had their own set of permissions, but these permissions were applied uniformly across all users within that tenant. This meant that if a feature was enabled for a tenant, every user in that tenant had access to it."
                            </p>
                            <p class="text-lg">
                                "As our user base grew, we quickly realized that this tenant-wide approach was too restrictive. Different users within the same tenant needed different levels of access - some needed full access to all features, while others needed limited access to specific tools. The existing system couldn't accommodate these varying needs."
                            </p>
                            <p class="text-lg">
                                "The new permission system was developed to solve this problem. It maintains the multi-tenant structure while adding the flexibility to manage permissions at the user level. This allows tenants to have different types of users with different access levels, making the platform more versatile and secure."
                            </p>
                            <p class="text-lg">
                                "Beyond user-level control, the system also enables fine-grained feature management. Instead of simply enabling or disabling entire sections, it can control specific behaviors and subsets within larger features. This means tenants can customize exactly how each feature works for different users, providing unprecedented flexibility in how the platform can be used. All these changes are applied in realtime, ensuring that users immediately see the effects of permission updates."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Architecture
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Technical Architecture"</h2>
                </div>
            </section>

            // Security & Implementation
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-8">
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"AWS Cognito Integration"</h3>
                                <p class="text-lg">
                                    "The system integrates with AWS Cognito for enhanced security. When a user is assigned the Tenant Admin role, they are automatically added to the `tenantAdmin` AWS Cognito user group. This provides an additional layer of security at the application routing level, ensuring that sensitive company settings are protected by both the permission system and AWS Cognito's route guards."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Key Reducer System"</h3>
                                <p class="text-lg">
                                    "Permissions are organized in a tree structure, with each permission's location defined by a key reducer - an array of strings that represents the path to the permission. This allows for precise targeting of permissions at any level of the tree, from broad feature access to specific actions within features. For example, a key reducer like `['core', 'contacts', 'delete']` can control the ability to delete contacts, while `['core', 'contacts', 'view']` controls viewing access."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Role-Based Strategy"</h3>
                                <p class="text-lg">
                                    "The system is designed to prioritize role-based permissions over user-specific permissions. This approach was chosen for its maintainability and scalability. When permissions are stored in roles, adding the same permissions to a new user is as simple as assigning the role, and updating permissions for multiple users requires changing just the role definition. User-specific permissions are treated as a last resort, used only when a single user needs unique access that doesn't fit into any existing role structure."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Cascading Permissions"</h3>
                                <p class="text-lg mb-6">
                                    "The permission system implements a cascading model where permissions flow from the most general to the most specific level. This creates a clear hierarchy of permission checks that determines the final access level for any user."
                                </p>
                                <div class="space-y-6">
                                    <div class="rounded-xl p-6 border bg-white shadow-sm">
                                        <h4 class="text-xl font-bold mb-3">
                                            "Permission Hierarchy"
                                        </h4>
                                        <div class="space-y-4">
                                            <div class="flex items-start">
                                                <div class="w-3 h-3 rounded-full mt-2 mr-3 border bg-white"></div>
                                                <div>
                                                    <p class="font-medium">"Tenant Access"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "The master switch that can override all other permissions. If a permission is set to false here, it will be false regardless of other settings."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-3 h-3 rounded-full mt-2 mr-3 border bg-white"></div>
                                                <div>
                                                    <p class="font-medium">"Industry Defaults"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Industry-specific permission overrides that can modify the base permissions for particular industries."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-3 h-3 rounded-full mt-2 mr-3 border bg-white"></div>
                                                <div>
                                                    <p class="font-medium">"Role-Based Permissions"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Permissions assigned through roles, with the Tenant Admin role having the highest priority, followed by user-defined roles in their specified order."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-3 h-3 rounded-full mt-2 mr-3 border bg-white"></div>
                                                <div>
                                                    <p class="font-medium">"User-Specific Permissions"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Individual user permissions that can override role-based permissions, used for exceptional cases where a user needs unique access."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-3 h-3 rounded-full mt-2 mr-3 border bg-white"></div>
                                                <div>
                                                    <p class="font-medium">"Code Defaults"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Base permissions baked into the code that serve as the default floor. These are used when no other permission level has explicitly set a value, ensuring there's always a defined permission state."
                                                    </p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                    <div class="rounded-xl p-6 border bg-white shadow-sm">
                                        <h4 class="text-xl font-bold mb-3">"How It Works"</h4>
                                        <p class="text-lg">
                                            "When checking a permission, the system evaluates each level in sequence. If a permission is explicitly set at any level, that value is used. If not, the system continues down the hierarchy until it finds a value or reaches the end. This ensures that more specific permissions can override more general ones, while maintaining a clear and predictable permission structure."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Recursive Implementation
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Recursive Implementation"</h2>
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-8">
                            <p class="text-lg">
                                "The permission system leverages recursion throughout its implementation, making it highly flexible and future-proof. This approach allows the system to handle permissions of any depth without requiring changes to the core functionality."
                            </p>
                            <div class="grid md:grid-cols-2 gap-8">
                                <div class="rounded-xl p-6 border bg-white shadow-sm">
                                    <h3 class="text-xl font-bold mb-4">"Recursive UI"</h3>
                                    <p class="mb-4">
                                        "The permission management interface uses a recursive component that can render permissions of any depth. Each level of the component handles both boolean values (permission toggles) and nested objects (permission groups), automatically creating new levels as needed."
                                    </p>
                                    <p>
                                        "This approach means that adding new permissions or deeper nesting in the future requires no changes to the UI code - the same component handles all levels of the permission tree."
                                    </p>
                                </div>
                                <div class="rounded-xl p-6 border bg-white shadow-sm">
                                    <h3 class="text-xl font-bold mb-4">"Recursive Types"</h3>
                                    <p class="mb-4">
                                        "The type system uses recursion to automatically generate types for the permission system. The `DeepPartial` type makes all keys optional, while `DeepPermissionsDetails` automatically converts boolean permissions into detailed permission objects."
                                    </p>
                                    <p>
                                        "This means developers can focus on defining the base permission structure, and the type system automatically handles the creation of all necessary types, reducing the chance of type mismatches."
                                    </p>
                                </div>
                            </div>
                            <div class="rounded-xl p-6 border bg-white shadow-sm">
                                <h3 class="text-xl font-bold mb-4">"Permission Merging"</h3>
                                <p class="mb-4">
                                    "The core of the permission system is the recursive merging function that combines permissions from different sources. This function can handle permissions of any depth, automatically merging them based on priority while maintaining the structure of the permission tree."
                                </p>
                                <p>
                                    "Built-in safeguards prevent infinite recursion, and the system tracks the depth of merging to ensure it doesn't exceed safe limits. This makes the system both powerful and safe, capable of handling complex permission scenarios while preventing potential issues."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Key Features
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">
                        "Key Features & Capabilities"
                    </h2>
                    <div class="grid md:grid-cols-3 gap-8">
                        <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm">
                            <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                <svg
                                    class="w-6 h-6"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Role Management"</h3>
                            <p>
                                "Flexible role system with static roles and user-defined roles. Roles can be dragged to set priority, and when permission values conflict, the role's priority determines the final compiled value."
                            </p>
                        </div>
                        <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm">
                            <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                <svg
                                    class="w-6 h-6"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Bulk Operations"</h3>
                            <p>
                                "Efficient user management through bulk role assignments and CSV uploads. The system supports multiple comma-separated roleIds per user and automatically matches case-variant role IDs during uploads."
                            </p>
                        </div>
                        <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm">
                            <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                <svg
                                    class="w-6 h-6"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Role Templates"</h3>
                            <p>
                                "Predefined role templates for consistent permission management across tenants. Templates are hard-coded in TypeScript files and can be selected during role creation, supporting various paradigms like job titles, locations, and payment tiers."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Development Journey
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Development Journey"</h2>
                    <div class="max-w-4xl mx-auto">
                        <div class="relative">
                            <div class="absolute left-8 top-0 bottom-0 w-0.5 border-l"></div>
                            <div class="space-y-12">
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">"System Design"</h3>
                                        <p>
                                            "Developed a cascading permissions model with tenant-wide controls, role-based access, and user-specific permissions. The system uses key reducers to locate permissions within the permission tree, ensuring flexible and secure access management."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">
                                            "Role Implementation"
                                        </h3>
                                        <p>
                                            "Implemented a sophisticated role system with static roles (Tenant Admin, Everyone) and user-defined roles. The Tenant Admin role controls the Tenant Admin user group and is locked at the top of the priority list, while the Everyone role provides baseline permissions."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">
                                            "Future Enhancements"
                                        </h3>
                                        <p>
                                            "Planning to implement permission change events and enhance the role template system. The system will support more complex organizational structures and provide better tools for managing permissions across multiple tenants."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
