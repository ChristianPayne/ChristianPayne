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
                            "A scalable permission management solution for Model Match that combines role-based access control, granular permissions, and real-time updates to provide precise control over user capabilities."
                        </p>
                        <div class="flex flex-wrap justify-center gap-4 mb-8">
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "TypeScript"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "DynamoDB"
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
                                "Developed as a core component of Model Match, this permission system was created to handle the complex access control needs of a multi-tenant platform. The initial system was too restrictive, applying the same access levels to all users within a tenant. This new system provides user-level control while maintaining the multi-tenant structure, allowing different users to have different access levels. All changes are applied instantly, and the flexible role system handles access management for users at every level of the organization."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // History & Motivation
            <section class="py-16 sm:py-24">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-6">
                            <h2 class="text-4xl font-bold mb-6">"History & Motivation"</h2>
                            <p class="text-lg">
                                "As Model Match expanded beyond individual loan officers to serve entire mortgage companies and real estate teams, companies reached out with needs for different access levels. Loan officers needed to see agent data and Market Insights, while business development teams needed broader market intelligence. The existing permission system couldn't handle these industry-specific role distinctions."
                            </p>
                            <p class="text-lg">
                                "Beyond simple on/off switches, the team realized that many features needed granular configuration. Users might need access to Market Insights but with limited data fields, or agents might not want access to loan data. The existing system couldn't handle these nuanced permission requirements."
                            </p>
                            <p class="text-lg">
                                "Additionally, some companies didn't want to pay for all the data features, so the team needed to section them off with their own sub-feature flags. The new permission system could control access to specific data sets, Market Insights, and tools, allowing organizations to build the right access levels for their team structure and budget."
                            </p>
                            <p class="text-lg">
                                "This led to a redesign focused on mortgage industry workflows. The new system could handle the complex permission needs of mortgage companies, from individual loan officers to enterprise teams, while maintaining the platform's core value proposition and enabling flexible pricing tiers."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Architecture
            <section class="py-16 sm:py-24">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-16">"Technical Architecture"</h2>
                    <div class="max-w-4xl mx-auto mb-12">
                        <p class="text-lg text-center">
                            "Understanding how the permission system works requires diving into its technical architecture. Here we'll explore the building blocks and see how they come together in the management interface."
                        </p>
                    </div>
                    <div class="max-w-4xl mx-auto">
                        <img
                            src="/images/permissions.png"
                            alt="Model Match permission management interface showing tenant access controls"
                            class="w-full max-w-4xl rounded-lg shadow-lg object-cover"
                        />
                    </div>
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
                                    <span>
                                        "AWS Cognito provides backup protection for the permission system. When someone becomes a Tenant Admin, they're automatically added to the "
                                    </span>
                                    <code class="bg-gray-100 px-2 py-1 rounded text-sm">
                                        "tenantAdmin"
                                    </code>
                                    <span>
                                        " AWS Cognito group. This creates an additional safety layer at the routing level, ensuring that company settings are protected by both the permission system and AWS Cognito's route guards."
                                    </span>
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Value Access"</h3>
                                <p class="text-lg mb-4">
                                    <span>
                                        "Permissions are organized in a tree structure, with each permission's location defined by a key reducer - an array of strings that represents an arbitrary location in the permission tree. This allows for precise targeting of permissions at any level, from broad feature access to specific actions within features. For example, a key reducer like "
                                    </span>
                                    <code class="bg-gray-100 px-2 py-1 rounded text-sm">
                                        "['core', 'contacts', 'delete']"
                                    </code>
                                    <span>
                                        " can control the ability to delete contacts, while "
                                    </span>
                                    <code class="bg-gray-100 px-2 py-1 rounded text-sm">
                                        "['core', 'contacts', 'view']"
                                    </code>
                                    <span>" controls viewing access."</span>
                                </p>
                                <p class="text-lg">
                                    "Sometimes layers of the permission system may not have a specified value for a particular location, so the key reducer method may not find anything there. This is expected behavior - the system gracefully handles missing permission definitions by falling back to code defaults, ensuring there's always a defined permission state."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Role-Based Strategy"</h3>
                                <p class="text-lg mb-4">
                                    "Roles were added to the system so permissions could be defined independently from users. When there's a group of users that commonly have the same role in a company, you can add the permissions onto the role and assign roles (or multiple roles) to users."
                                </p>
                                <p class="text-lg mb-4">
                                    "Roles have a hierarchy that can be changed on the permissions page to give you control over cascading. When two or more roles have the same permissions specified, we would have a conflict. The order you put the roles in on the permissions system management page allows us to know how merge conflicts should be handled."
                                </p>
                                <p class="text-lg">
                                    "User-specific permissions are treated as the most specific level, used when a single user needs unique access that doesn't fit into any existing role structure. This approach was chosen for its maintainability and scalability - when permissions are stored in roles, adding the same permissions to a new user is as simple as assigning the role, and updating permissions for multiple users requires changing just the role definition."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Cascading Permissions"</h3>
                                <p class="text-lg mb-6">
                                    "Permissions cascade through the system like a waterfall, flowing from the most general level down to the most specific. This means you can set broad permissions at the tenant level and then override them with more specific settings at the role or user level. The system automatically resolves conflicts by using the most specific permission value it finds, ensuring predictable behavior."
                                </p>
                                <p class="text-lg mb-6">
                                    "The permission hierarchy consists of five distinct levels, each with its own purpose and priority. Understanding how these levels interact is key to effectively managing permissions across the platform."
                                </p>
                                <div class="space-y-6">
                                    <div class="rounded-xl p-6 border bg-white shadow-sm">
                                        <h4 class="text-xl font-bold mb-3">
                                            "Permission Hierarchy"
                                        </h4>
                                        <div class="space-y-4">
                                            <div class="flex items-start">
                                                <div class="w-2 h-2 rounded-full mt-2 mr-3 border bg-white flex-shrink-0"></div>
                                                <div>
                                                    <p class="font-medium">"Tenant Access"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "The master switch that can override all other permissions. If a permission is set to false here, it will be false regardless of other settings. Think of it as a circuit breaker on the outside of a house - if this turns off, everything downstream of it also gets shut off."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-2 h-2 rounded-full mt-2 mr-3 border bg-white flex-shrink-0"></div>
                                                <div>
                                                    <p class="font-medium">"Industry Defaults"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Industry-specific permission overrides that can modify the base permissions for particular industries. Sometimes we work with other industries or we might in the future. This was mainly a future-proofing step that gave us code defaults for whole industry tenant access. Some features, like loan data, just are not relevant in some industries, so we can turn them off for groups of tenants without having to go into each tenant and tune industry-specific features."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="border-t border-gray-200 my-4"></div>
                                            <div class="flex items-start">
                                                <div class="w-2 h-2 rounded-full mt-2 mr-3 border bg-white flex-shrink-0"></div>
                                                <div>
                                                    <p class="font-medium">"User-Specific Permissions"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Individual user permissions that can override role-based permissions, used for exceptional cases where a user needs unique access. This level allows administrators to grant specific permissions to individual users that don't fit into any existing role structure. These permissions are the most specific and take precedence over role-based permissions."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-2 h-2 rounded-full mt-2 mr-3 border bg-white flex-shrink-0"></div>
                                                <div>
                                                    <p class="font-medium">"Role-Based Permissions"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Permissions assigned through roles, with the Tenant Admin role having the highest priority, followed by user-defined roles in their specified order. The system includes static roles like Tenant Admin and Everyone, plus user-defined roles that can be created to match organizational needs. When multiple roles have conflicting permissions, the order specified on the permissions management page determines which one takes precedence."
                                                    </p>
                                                </div>
                                            </div>
                                            <div class="flex items-start">
                                                <div class="w-2 h-2 rounded-full mt-2 mr-3 border bg-white flex-shrink-0"></div>
                                                <div>
                                                    <p class="font-medium">"Code Defaults"</p>
                                                    <p class="text-sm text-gray-600">
                                                        "Base permissions baked into the code that serve as the default floor. These are used when no other permission level has explicitly set a value, ensuring there's always a defined permission state. This level acts as the final fallback in the permission hierarchy, guaranteeing that every permission check returns a definitive true or false value."
                                                    </p>
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Recursive Implementation
            <section class="py-16 sm:py-24">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Recursive Implementation"</h2>
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-8">
                            <p class="text-lg">
                                "NEEDS REVIEW: The permission system leverages recursion throughout its implementation, making it highly flexible and future-proof. This approach allows the system to handle permissions of any depth without requiring changes to the core functionality."
                            </p>
                            <div class="grid md:grid-cols-2 gap-8">
                                <div class="rounded-xl p-6 border bg-white shadow-sm">
                                    <h3 class="text-xl font-bold mb-4">"Recursive UI"</h3>
                                    <p class="mb-4">
                                        "NEEDS REVIEW: The permission management interface uses a recursive component that can render permissions of any depth. Each level of the component handles both boolean values (permission toggles) and nested objects (permission groups), automatically creating new levels as needed."
                                    </p>
                                    <p>
                                        "NEEDS REVIEW: This approach means that adding new permissions or deeper nesting in the future requires no changes to the UI code - the same component handles all levels of the permission tree."
                                    </p>
                                </div>
                                <div class="rounded-xl p-6 border bg-white shadow-sm">
                                    <h3 class="text-xl font-bold mb-4">"Recursive Types"</h3>
                                    <p class="mb-4">
                                        "NEEDS REVIEW: The type system uses recursion to automatically generate types for the permission system. The `DeepPartial` type makes all keys optional, while `DeepPermissionsDetails` automatically converts boolean permissions into detailed permission objects."
                                    </p>
                                    <p>
                                        "NEEDS REVIEW: This means developers can focus on defining the base permission structure, and the type system automatically handles the creation of all necessary types, reducing the chance of type mismatches."
                                    </p>
                                </div>
                            </div>
                            <div class="rounded-xl p-6 border bg-white shadow-sm">
                                <h3 class="text-xl font-bold mb-4">"Permission Merging"</h3>
                                <p class="mb-4">
                                    "NEEDS REVIEW: The core of the permission system is the recursive merging function that combines permissions from different sources. This function can handle permissions of any depth, automatically merging them based on priority while maintaining the structure of the permission tree."
                                </p>
                                <p>
                                    "NEEDS REVIEW: Built-in safeguards prevent infinite recursion, and the system tracks the depth of merging to ensure it doesn't exceed safe limits. This makes the system both powerful and safe, capable of handling complex permission scenarios while preventing potential issues."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Key Features
            <section class="py-16 sm:py-24">
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
                                "NEEDS REVIEW: Flexible role system with static roles and user-defined roles. Roles can be dragged to set priority, and when permission values conflict, the role's priority determines the final compiled value."
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
                                "NEEDS REVIEW: Efficient user management through bulk role assignments and CSV uploads. The system supports multiple comma-separated roleIds per user and automatically matches case-variant role IDs during uploads."
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
                                "NEEDS REVIEW: Predefined role templates for consistent permission management across tenants. Templates are hard-coded in TypeScript files and can be selected during role creation, supporting various paradigms like job titles, locations, and payment tiers."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Development Journey
            <section class="py-16 sm:py-24">
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
                                            "NEEDS REVIEW: Developed a cascading permissions model with tenant-wide controls, role-based access, and user-specific permissions. The system uses key reducers to locate permissions within the permission tree, ensuring flexible and secure access management."
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
                                            "NEEDS REVIEW: Implemented a sophisticated role system with static roles (Tenant Admin, Everyone) and user-defined roles. The Tenant Admin role controls the Tenant Admin user group and is locked at the top of the priority list, while the Everyone role provides baseline permissions."
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
                                            "NEEDS REVIEW: Planning to implement permission change events and enhance the role template system. The system will support more complex organizational structures and provide better tools for managing permissions across multiple tenants."
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
