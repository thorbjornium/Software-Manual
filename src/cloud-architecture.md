# Cloud Architecture

<!-- toc -->

## **As a Service Models**

IaaS (Infrastructure as a Service):

- IaaS provides virtualized computing resources over the internet. Users can rent servers, storage, networking, and other infrastructure components rather than purchasing and maintaining physical hardware.
- With IaaS, users have control over the operating systems, applications, and development frameworks, while the cloud provider manages the underlying infrastructure.
- Popular examples of IaaS providers include Amazon Web Services (AWS), Microsoft Azure, and Google Cloud Platform (GCP).

PaaS (Platform as a Service):

- PaaS provides a platform allowing customers to develop, run, and manage applications without dealing with the complexity of building and maintaining the underlying infrastructure.
- It typically includes tools and services for application development, such as databases, middleware, development tools, and operating systems.
- PaaS abstracts away much of the underlying infrastructure management, allowing developers to focus more on coding and deploying their applications.
- Examples of PaaS offerings include Heroku, Google App Engine, and Microsoft Azure App Service.

SaaS (Software as a Service):

- SaaS delivers software applications over the internet on a subscription basis. Users access these applications via a web browser or API without needing to install or maintain any software locally.
- SaaS applications are hosted and managed by the service provider, who handles maintenance, updates, security, and scalability.
- Examples of SaaS applications include Salesforce for CRM, Google Workspace for productivity tools, and Netflix for streaming media.

FaaS (Function as a Service):

- FaaS is a cloud computing model where cloud providers manage the infrastructure required to execute code in response to events or triggers.
- Developers write small, single-purpose functions, and the cloud provider dynamically allocates resources to execute these functions as needed.
- FaaS abstracts away the need for developers to manage servers, scaling, and infrastructure, allowing them to focus solely on writing code.
- Examples of FaaS platforms include AWS Lambda, Google Cloud Functions, and Microsoft Azure Functions.

### **Additional resources**

What are IaaS, PaaS, and SaaS: [https://www.ibm.com/topics/iaas-paas-saas](https://www.ibm.com/topics/iaas-paas-saas)

## **Shared Responsibility Model**

After completing this episode, you should be able to:

Discuss the shared responsibility model as it exists in modern cloud architectures

**Description:** In this episode, you will learn about the shared responsibility model. This model describes the shared responsibilities that cloud providers and users of the cloud must bear in their solutions.

### **Shared Responsibility Model**

Shared responsibility model - the shared responsibility model in cloud computing delineates the responsibilities between the cloud service provider (CSP) and the customer in terms of security and management of resources. In this model, the CSP is responsible for securing the underlying infrastructure, including physical data centers, networking, and hypervisors, as well as ensuring the availability and reliability of the cloud services. On the other hand, the customer is responsible for securing their data, applications, identities, and configurations within the cloud environment. This entails tasks such as configuring access controls, implementing encryption, managing user permissions, and securing application code. Essentially, while the CSP manages the security of the cloud infrastructure, the customer is accountable for securing their data and applications within that infrastructure, forming a collaborative approach to ensuring overall security and compliance in the cloud.

### **Additional resources**

Shared Responsibility Model: [https://www.crowdstrike.com/cybersecurity-101/cloud-security/shared-responsibility-model/](https://www.crowdstrike.com/cybersecurity-101/cloud-security/shared-responsibility-model/)

## **Resource Availability**

After completing this episode, you should be able to:

Discuss the topic of resource availability as it relates to modern day cloud technologies

**Description:** In this episode, you will learn about the topic of resource availability when it comes to cloud architectures. This discussion includes such topics as regions, availability zones, cloud bursting, edge computing, and availability monitoring.

### **Resource Availability**

Regions - the major public cloud providers (Amazon, Google, Microsoft) offer global infrastructures that span the entire globe. They divide these resources into regions. Customers can place resources in specific regions in order to reduce latency on access.

Availability zones - an availability zone is a distinct and isolated location within a geographic region that houses data centers equipped with independent power, cooling, and networking infrastructure. Availability zones are designed to provide redundancy and fault tolerance, enabling applications and services to remain operational even in the event of failures or disruptions affecting one availability zone.

Cloud bursting - cloud bursting is a dynamic and flexible cloud computing strategy that allows organizations to seamlessly scale their workloads from a private cloud to a public cloud environment during periods of peak demand. In essence, it enables businesses to augment their existing on-premises infrastructure with additional computing resources from a public cloud provider when local capacity is exceeded. This burst capacity can accommodate sudden spikes in user activity, seasonal variations, or other unforeseen surges in demand without requiring significant upfront investments in infrastructure expansion. By leveraging cloud bursting, organizations can optimize resource utilization, maintain performance levels, and ensure uninterrupted service delivery, all while minimizing costs and maximizing operational efficiency.

Edge computing - Edge computing is a distributed computing paradigm that brings computational resources closer to the data source or end-user devices, thereby reducing latency and improving real-time processing capabilities. Unlike traditional cloud computing, where data is processed in centralized data centers, edge computing decentralizes computing power to the "edge" of the network, such as IoT devices, routers, or local servers. This approach enables faster data processing, analysis, and response times, making it ideal for applications requiring low latency, high bandwidth, and efficient use of network resources. Edge computing is particularly valuable in scenarios like IoT deployments, autonomous vehicles, and industrial automation, where timely decision-making and local data processing are critical. By pushing computing closer to where data is generated and consumed, edge computing enhances scalability, reliability, and security while unlocking new opportunities for innovation and efficiency in diverse industries.

Availability monitoring - cloud computing makes it easy to provide availability monitoring for both your own cloud workloads, as well as the services of the cloud provider. While availability monitoring for your resources in the cloud requires some level of configuration, the major cloud providers all provide web pages (and even APIs) that provide availability information for their resources and services.

### **Additional resources**

Cloud Bursting: [https://www.vmware.com/topics/glossary/content/cloud-bursting.html](https://www.vmware.com/topics/glossary/content/cloud-bursting.html)

## **Disaster Recovery**

After completing this episode, you should be able to:

Discuss the topic of disaster recovery (DR) as it applies to cloud technology

**Description:** In this episode, you will learn about the topic of disaster recovery (DR) as it applies to cloud technology. This includes topics such as recovery time objective (RTO), recovery point objective (RPO), host site, warm site, and cold site.

### **Disaster Recovery**

Recovery Time Objective (RTO) - A Recovery Time Objective (RTO) is the targeted duration within which a business process or service must be restored after a disruption or outage. It represents the maximum acceptable downtime tolerated by an organization and is typically defined in terms of hours, minutes, or seconds.

Recovery Point Objective (RPO) - Recovery Point Objective (RPO) is the maximum tolerable amount of data loss measured in time before a disruption or outage. It represents the point in time to which data must be recovered after an incident, and is typically expressed in terms of minutes, hours, or days.

Hot site - A hot site is a fully operational backup facility equipped with necessary infrastructure and systems to resume business operations immediately after a disaster. It typically mirrors the primary site's computing environment and data in real-time or near-real-time, ensuring minimal downtime and data loss in the event of a disruption.

Warm site - A warm site is a backup facility that contains some of the necessary infrastructure and data required to resume business operations after a disaster, but it is not fully operational in real-time like a hot site. It requires additional setup and configuration before it can become fully functional, leading to a longer recovery time compared to a hot site.

Cold site - A cold site is a backup facility that provides only basic infrastructure such as power and physical space, with no pre-installed equipment or data. It requires substantial setup and configuration in the event of a disaster, resulting in the longest recovery time compared to hot and warm sites.

### **Additional resources**

Cloud DR: [https://www.techtarget.com/searchdisasterrecovery/definition/cloud-disaster-recovery-cloud-DR](https://www.techtarget.com/searchdisasterrecovery/definition/cloud-disaster-recovery-cloud-DR)

## **Multicloud Tenancy**

After completing this episode, you should be able to:

Discuss the topic of multicloud tenancy and describe its major aspects

**Description:** In this episode, you will learn about the topic of multicloud tenancy. This includes a discussion of its many potential benefits.

### **Multicloud Tenancy**

Multicloud tenancy is when you use the services of multiple public or private cloud providers. This provides you with many potential advantages, including:

- Optimal service selection you can choose the solution that features the parameters you need; possible parameters include speed, performance, reliability, geographical location, and security and compliance requirements
- Avoid vendor lock-in thanks to using multiple cloud providers, you are not locked in to a single vendor
- Cost efficiency you can take advantage of the best possible price for services as you have multiple providers to choose from
- Innovative technology you can take advantage of new technologies that might not yet be offered by other cloud providers
- Advanced security you can take advantage of the latest in security and compliance technologies
- Increased reliability since there is not single point of failure, your reliability posture improves

Some classic use cases for multicloud include:

- Disaster recovery
- Improved latency
- Regional requirements
- Reduction in shadow IT

### **Additional resources**

What is Multicloud: [https://cloud.google.com/learn/what-is-multicloud](https://cloud.google.com/learn/what-is-multicloud)

## **Connections to the Cloud**

After completing this episode, you should be able to:

Discuss the various options for connecting your local network to the cloud

**Description:** In this episode, you will learn about the many options that exist for connecting to your cloud resources. These many options vary from a simple public Internet connection to a completely private, direct connection to the cloud.

### **Connections to the Cloud**

There are many options when needing connectivity to your cloud resources - these include:

- Public Internet connection A public Internet connection to the public cloud utilizes the existing infrastructure of the internet to establish connectivity between the user's local network and the cloud provider's data centers, offering accessibility and convenience for accessing cloud resources from virtually any location. While cost-effective and widely available, public Internet connections may present security and performance challenges compared to dedicated or private connectivity options.
- Software VPN solution A software VPN connection to the public cloud creates a secure, encrypted tunnel over the internet, enabling users to securely access cloud resources from remote locations with authentication and encryption protocols. This flexible and scalable solution offers cost-effective connectivity while ensuring data privacy and integrity for organizations leveraging cloud services.
- Hardware VPN solution A hardware VPN solution for connecting to the public cloud involves deploying dedicated networking appliances or routers equipped with VPN capabilities at both the on-premises and cloud ends, ensuring secure communication via encrypted tunnels over the internet. This approach offers enhanced performance, scalability, and reliability compared to software VPNs, making it suitable for organizations with high-volume traffic and stringent security requirements.
- Private, dedicated connection A private, dedicated connection to the public cloud establishes a direct physical link between the organization's on-premises infrastructure and the cloud provider's data centers, bypassing the public internet for enhanced security and reliability. This high-bandwidth, low-latency connection ensures consistent performance and enables seamless integration of on-premises and cloud resources, ideal for mission-critical applications and sensitive workloads.

### **Additional resources**

Amazon Virtual Private Cloud Connectivity Options: [https://docs.aws.amazon.com/whitepapers/latest/aws-vpc-connectivity-options/introduction.html](https://docs.aws.amazon.com/whitepapers/latest/aws-vpc-connectivity-options/introduction.html)

## **Network Components**

After completing this episode, you should be able to:

Discuss the various network components that are made available in the cloud

**Description:** In this episode, you will learn about just some of the network components that are made available virtually in the cloud. These components include application load balancers, network load balancers, application gateways, content delivery networks (CDNs), and firewalls.

### **Network Components**

Application load balancer - A cloud-based application load balancer is a scalable, resilient service deployed in the cloud environment that distributes incoming traffic across multiple backend servers or instances to optimize performance and ensure high availability. By intelligently routing requests based on factors such as server health, proximity, and application-specific rules, it enhances the overall reliability and responsiveness of cloud-based applications.

Network load balancer - A cloud-based network load balancer is a virtual appliance or service deployed in the cloud environment that evenly distributes incoming network traffic across multiple backend resources, such as virtual machines or containers, to ensure efficient utilization and high availability. Leveraging advanced routing algorithms and monitoring capabilities, it dynamically adjusts traffic flow to maintain optimal performance and minimize downtime, enhancing the scalability and reliability of cloud-based networks.

Application gateway - A cloud-based application gateway serves as a centralized entry point for accessing web applications hosted in the cloud, providing secure and controlled access to these applications from external networks. By offering features such as SSL termination, web application firewall (WAF), and content caching, it enhances security, accelerates performance, and simplifies application delivery for users accessing cloud-hosted services.

Content delivery network (CDN) - A content delivery network (CDN) is a distributed network of servers strategically placed across various geographic locations to deliver web content, such as images, videos, and static files, to users with optimal speed and reliability. By caching content closer to end-users and dynamically routing requests based on proximity and server availability, CDNs reduce latency, alleviate network congestion, and improve the overall performance and scalability of websites and web applications.

Firewalls - A cloud-based firewall is a security service deployed in the cloud environment to monitor, filter, and control incoming and outgoing network traffic based on predefined security policies. By inspecting traffic at the application layer and enforcing security rules, it protects cloud-hosted resources from unauthorized access, malicious attacks, and data breaches, providing enhanced security posture and compliance assurance for organizations leveraging cloud infrastructure.

### **Additional resources**

Azure Front Door: [https://azure.microsoft.com/en-us/products/frontdoor](https://azure.microsoft.com/en-us/products/frontdoor)

## **Network Services**

After completing this episode, you should be able to:

Discuss the various network services that are typically made available in cloud

**Description:** In this episode, you will learn about just some of the network services that are made available virtually in the cloud. These components include virtual private clouds, subnets, VLANs, and more.

### **Network Services**

Virtual Private Cloud - A Virtual Private Cloud (VPC) is a logically isolated section of a public cloud provider's infrastructure where users can deploy their virtual networks, subnets, and resources with customizable network configurations. By offering dedicated resources and network segmentation, VPCs enable organizations to create secure and scalable environments within the public cloud, ensuring data privacy, network isolation, and controlled access to cloud resources.

VPC Peering - VPC peering in cloud networking enables direct communication between Virtual Private Clouds (VPCs) within the same cloud provider's infrastructure, allowing seamless exchange of traffic without traversing the public internet. This facilitates interconnectivity between different VPCs, enabling organizations to build complex architectures and share resources while maintaining network isolation and security boundaries.

VPC Transit Gateway - A VPC transit gateway in cloud networking acts as a centralized hub that facilitates communication between multiple Virtual Private Clouds (VPCs) and on-premises networks, streamlining network connectivity and management across complex architectures. By providing scalable and efficient routing between interconnected networks, VPC transit gateways simplify network configuration, improve performance, and enhance security in cloud environments.

Subnets - A subnet in cloud networking is a segmented portion of an IP network, typically within a Virtual Private Cloud (VPC), that contains a subset of available IP addresses reserved for specific purposes or resource allocation. Subnets enable organizations to organize and manage their cloud resources efficiently, facilitating network segmentation, security, and efficient resource utilization within the cloud environment.

VLANs - VLANs, or Virtual Local Area Networks, in cloud networking are logical segmentation of a physical network into distinct broadcast domains, allowing for isolation and control of network traffic within a cloud environment. By assigning devices to VLANs based on their function, VLANs facilitate efficient resource utilization, security policies, and network management in cloud-based infrastructures.

SDN - Software-Defined Networking (SDN) is a networking approach that separates the control plane from the data plane, allowing network administrators to centrally manage and orchestrate network resources through software-based controllers. By decoupling network intelligence from physical infrastructure, SDN enables dynamic and programmable network configurations, improving agility, scalability, and automation in modern network environments.

BGP - BGP, or Border Gateway Protocol, is a standardized routing protocol used to exchange routing information between different autonomous systems (ASes) on the internet. It enables routers to dynamically learn and advertise the best paths to reach destination networks, facilitating scalable and efficient routing across diverse network topologies.

Static routes - Static routes are manually configured routes in a network device's routing table that specify how to reach specific destination networks. Unlike dynamic routing protocols, static routes do not dynamically adapt to changes in network topology and require manual intervention to update, making them suitable for simple network configurations with stable routes.

Route tables - Route tables are tables maintained by network devices that contain information about the best paths to reach destination networks, including next-hop routers and interface associations. By using routing protocols or static configurations, route tables enable routers to make forwarding decisions and efficiently route packets through the network.

### **Additional resources**

Amazon Virtual Private Cloud: [https://aws.amazon.com/vpc/](https://aws.amazon.com/vpc/)

## **Disk and Storage Types**

After completing this episode, you should be able to:

Discuss some of the various disk and storage types that are available in the cloud

**Description:** In this episode, you will learn about just some of the disk and storage types available in the cloud today. This episode includes a discussion of object storage, block storage, and file storage as well as solid-state drives and hard disk drives.

### **Disk and Storage Types**

Storage types:

- Object storage Object-based storage is a data storage architecture that organizes and manages data as objects, each with a unique identifier, metadata, and data, providing scalability, flexibility, and simplified management for handling vast amounts of unstructured data.
- Block storage Block-based storage is a data storage approach that divides data into fixed-sized blocks, typically accessed via protocols like iSCSI or Fibre Channel, allowing for direct and low-level access to storage volumes and enabling highperformance storage solutions for applications like databases and virtualization.
- File storage File-based storage is a data storage method where data is organized into files and directories, accessible via file protocols such as NFS or SMB, offering a hierarchical structure for storing and accessing data in a shared environment.

### Disk types

- Solid-state drive (SSD) SSD is a storage device that uses flash memory technology to store data persistently, offering faster access times, lower latency, and higher reliability compared to traditional hard disk drives (HDDs).
- Hard disk drive (HDD) HDD is a storage device that uses rotating magnetic disks and mechanical read/write heads to store and retrieve data persistently, providing high-capacity storage at a lower cost per gigabyte compared to solid-state drives (SSDs).

Performance implications - Choosing between different cloud-based storage options such as object-based storage, block-based storage, and file-based storage can significantly impact performance, as each option offers distinct characteristics tailored to specific workload requirements. Object-based storage excels in handling large amounts of unstructured data with scalability, while block-based storage provides high-performance storage for transactional workloads, and file-based storage offers compatibility with legacy applications and file-sharing capabilities.

Cost implications - Selecting different cloud-based storage options, like object-based storage, block-based storage, and file-based storage, can influence costs based on factors such as storage capacity, data access frequency, and additional features like redundancy and data replication. Object-based storage typically offers cost-effective scalability for large volumes of data, while block-based storage may incur higher costs for provisioned storage and data transfer, and file-based storage often includes pricing based on capacity and access frequency.

### **Additional resources**

What is Cloud Storage: [https://aws.amazon.com/what-is/cloud-storage/](https://aws.amazon.com/what-is/cloud-storage/)

## **Tiered Storage**

After completing this episode, you should be able to:

Discuss storage tiering in cloud environments

**Description:** In this episode, you will learn about the ability to tier storage using cloud-based technologies. This includes a discussion of various tiers including hot, warm, cold, and archive storage.

### **Tiered Storage**

Storage tiering in cloud environments involves categorizing data based on its access frequency and performance requirements, with different tiers offering varying levels of performance and cost. By dynamically moving data between tiers based on usage patterns, organizations can optimize storage costs while ensuring that data remains accessible and responsive to application needs.

Typical storage tiers include:

- Hot A hot storage tier refers to a high-performance storage layer optimized for frequently accessed data that requires lowlatency access and high throughput.
- Warm A warm storage tier denotes a storage layer designed for data that is accessed less frequently than hot data but still requires moderate performance and relatively fast access times.
- Cold A cold storage tier is a low-cost storage layer intended for long-term retention of infrequently accessed data that prioritizes cost-efficiency over immediate accessibility or performance.
- Archive An archive storage tier is a cost-effective storage solution designed for long-term retention of data that is rarely accessed and requires minimal performance, often with additional features for compliance and data preservation.

### **Additional resources**

Storage Classes: [https://cloud.google.com/storage/docs/storage-classes](https://cloud.google.com/storage/docs/storage-classes)

## **Managed Services**

After completing this episode, you should be able to:

Discuss the use of managed services in modern cloud architectures

**Description:** In this episode, you will learn about the use of managed services in modern cloud architectures. This episode covers the concepts and advantages presented by this approach.

### **Managed Services**

Managed services in cloud environments refer to outsourced cloud computing solutions where cloud providers assume responsibility for the operation, maintenance, and management of specific IT functions or applications. This allows organizations to offload tasks such as database management, security monitoring, and infrastructure provisioning to cloud providers, reducing operational overhead, improving scalability, and enabling focus on core business activities.

Using managed services in cloud architectures offers several advantages:

- Reduced Operational Overhead: Managed services offload routine tasks such as infrastructure management, patching, and backups to cloud providers, freeing up internal resources to focus on strategic initiatives and core business activities.
- Scalability and Flexibility: Managed services typically offer scalable resources and flexible pricing models, allowing organizations to quickly adapt to changing workload demands without upfront investment in hardware or infrastructure.
- Improved Performance and Reliability: Cloud providers leverage their expertise and economies of scale to deliver managed services with high availability, reliability, and performance, backed by service level agreements (SLAs) that ensure uptime and quality of service.
- Enhanced Security and Compliance: Managed services often include built-in security features, compliance certifications, and regular security updates, helping organizations mitigate security risks, adhere to regulatory requirements, and protect sensitive data.
- Access to Advanced Technologies: Cloud providers continuously innovate and introduce new technologies and features to their managed services offerings, enabling organizations to leverage cutting-edge tools and capabilities without the burden of managing and maintaining them internally.
- Streamlined Management and Automation: Managed services typically come with centralized management consoles, automation tools, and APIs that simplify deployment, monitoring, and management of cloud resources, reducing complexity and improving operational efficiency.
- Cost Optimization: By leveraging managed services, organizations can benefit from pay-as-you-go pricing models, cost-effective resource utilization, and economies of scale offered by cloud providers, resulting in lower total cost of ownership (TCO) compared to traditional on-premises solutions.
- Focus on Core Competencies: Outsourcing non-core functions to managed services allows organizations to focus on their core competencies and strategic objectives, driving innovation, growth, and competitive advantage in their respective industries.

### **Additional resources**

Managed Cloud Services - Pros and Cons: [https://cloudian.com/guides/hybrid-cloud/managed-cloud-services/](https://cloudian.com/guides/hybrid-cloud/managed-cloud-services/)

## **Microservices**

After completing this episode, you should be able to:

Discuss the use of microservices in modern cloud environments

**Description:** In this episode, you will learn about the use of microservices in a cloud environment. This includes the concepts of loosely coupled architectures, fan-out, and service discovery services.

### **Microservices**

Microservices architecture is a software development approach where applications are composed of small, independent services that communicate with each other through well-defined APIs. Each service is focused on a specific business function and can be developed, deployed, and scaled independently. This modular approach allows for greater flexibility, resilience, and scalability compared to monolithic architectures. Microservices promote agility by enabling teams to work on different services simultaneously, using diverse technologies and programming languages best suited to each task. Additionally, they facilitate continuous integration and delivery practices, allowing for faster and more frequent updates with minimal disruption. However, managing a distributed system of microservices requires robust orchestration, monitoring, and governance to ensure seamless operation and maintainability.

Loosely coupled architecture in microservices allows each service to operate independently, minimizing dependencies between components. This enhances flexibility and scalability, enabling rapid development and deployment of new features without impacting other parts of the system.

A fan-out architecture in microservices involves distributing a single request to multiple services to perform parallel processing. This approach improves system performance and scalability by allowing each service to focus on its specialized task, thereby reducing overall response time and increasing throughput.

Service discovery in a microservices architecture is the process of dynamically finding and locating available services within the system. It involves mechanisms and tools that enable services to register themselves and discover other services they depend on. This allows for seamless communication and interaction between services, regardless of their location or deployment status, fostering flexibility, resilience, and scalability in the overall system.

### **Additional resources**

Microservices: [https://aws.amazon.com/microservices/](https://aws.amazon.com/microservices/)

## **Stand-alone vs Orchestrated**

After completing this episode, you should be able to:

Discuss stand-alone vs orchestrated container implementations

**Description:** In this episode, you will learn about the different methods of implementing containers in your cloud environment. This discussion includes a comparison of stand-alone versus orchestrated cloud container infrastructures.

### **Stand-alone vs Orchestrated**

In a standalone container infrastructure, each application or service runs within its isolated container, managed and operated individually. This means that each container must be manually configured, deployed, and scaled, typically without coordination with other containers. While standalone containers offer simplicity and flexibility, they require more manual effort to manage and lack advanced features like automated scaling and load balancing.

On the other hand, an orchestrated container infrastructure utilizes a container orchestration platform such as Kubernetes to automate the management, deployment, scaling, and networking of containers across a cluster of machines. With container orchestration, administrators can define desired states for the application, such as the number of instances, resource requirements, and networking rules. The orchestration platform then handles the deployment and scaling of containers to meet these requirements automatically. This approach simplifies operations, improves resource utilization, and enhances fault tolerance by automatically redistributing workloads and restarting containers in case of failures.

While standalone containers offer simplicity and are suitable for smaller deployments or environments where manual management is acceptable, orchestrated container infrastructures are preferred for larger-scale deployments, where automation, scalability, and reliability are critical requirements. They provide more advanced features and capabilities to manage complex distributed applications efficiently.

### **Additional resources**

What is a Container: [https://www.docker.com/resources/what-container/](https://www.docker.com/resources/what-container/)

## **Supporting Containers**

After completing this episode, you should be able to:

Discuss networking, storage, and registries typically found in container environments

**Description:** In this episode, you will learn about some of the different options when it comes to supporting container environments. This video includes information on port mapping, persistent volumes, ephemeral storage, and image registries.

### **Supporting Containers**

Container networking involves establishing communication between containers running on the same host or across multiple hosts. One essential aspect of container networking is port mapping, which allows containers to expose and access services running inside them or in other containers.

Port mapping works by mapping a port on the host system to a port inside the container. This allows external traffic to reach the containerized service by accessing the corresponding port on the host. Conversely, it enables services inside containers to communicate with each other or with services running on the host or other containers.

For example, if a web server container exposes port 80 internally for HTTP traffic, you can map port 80 of the container to port 8080 on the host. Any requests made to port 8080 on the host will be forwarded to port 80 inside the container, allowing external users to access the web server.

Port mapping is typically configured during container creation using containerization platforms like Docker or Kubernetes. In Docker, you can specify port mappings using the -p or --publish flag when running a container. In Kubernetes, you define port mappings in the pod or service configurations.

Port mapping plays a vital role in container networking by enabling containers to expose services to the outside world and facilitating communication between containers and external systems. It allows for flexible and secure deployment of containerized applications while ensuring seamless connectivity and interoperability in distributed environments.

In a typical cloud infrastructure, container storage involves managing data persistence and storage for containerized applications. This includes two main components: persistent volumes and ephemeral storage.

Persistent Volumes: Persistent volumes are storage volumes that exist independently of containers and can be dynamically provisioned and attached to containers as needed. They provide a way to store and persist data beyond the lifecycle of individual containers. Persistent volumes are commonly used for storing application data, databases, or shared files that need to be retained even if the container is restarted or recreated. Cloud providers offer various options for persistent volumes, including block storage (e.g., AWS EBS, Azure Disk), file storage (e.g., AWS EFS, Azure Files), or object storage (e.g., AWS S3, Azure Blob Storage). Kubernetes, for example, allows administrators to define persistent volume claims (PVCs) that specify storage requirements, and the underlying infrastructure dynamically provisions and attaches the appropriate persistent volume to the container.

Ephemeral Storage: Ephemeral storage refers to temporary storage that is directly associated with the container instance and exists only for the duration of the container's lifecycle. Ephemeral storage is typically used for storing transient data, such as application logs, temporary files, or cache data, that does not need to be preserved beyond the container's lifetime. Ephemeral storage is usually provided as part of the container runtime environment and is stored on the underlying host's filesystem. In cloud environments, ephemeral storage may be backed by local SSDs or instance storage, offering fast access and low latency.

Image repositories (or registries) play a central role in a container-based infrastructure by serving as a centralized location for storing, distributing, and managing container images. These repositories store the immutable images that contain everything needed to run a containerized application, including the application code, runtime environment, dependencies, and configurations. Here's how image repositories are used in a container-based infrastructure:

Image Distribution: Container images are typically built and stored in a registry or repository, such as Docker Hub, Google Container Registry, or Amazon Elastic Container Registry. These repositories provide a secure and scalable platform for storing and distributing container images globally. Developers can push newly built images to the repository, making them available for deployment across different environments.

Version Control: Image repositories support versioning, allowing developers to track changes and updates to container images over time. Each image version is tagged with a unique identifier, such as a version number or a git commit hash, enabling precise control and management of image versions. Versioning ensures consistency and reproducibility in deployments, as developers can deploy specific versions of container images with confidence.

Image Lifecycle Management: Image repositories facilitate the management of the container image lifecycle, including image creation, storage, deletion, and garbage collection. Administrators can define policies and rules for image retention and cleanup, ensuring efficient use of storage resources and compliance with organizational requirements. Automated tools and workflows can be integrated with the repository to streamline image management tasks and enforce best practices.

Access Control and Security: Image repositories support access control mechanisms to regulate who can push, pull, or modify images stored in the repository. Role-based access control (RBAC), authentication, and authorization mechanisms help enforce security policies and prevent unauthorized access or tampering with images. Additionally, image repositories may offer scanning and vulnerability

detection features to identify and mitigate security risks in container images before deployment.

Integration with Orchestration Platforms: Container orchestration platforms like Kubernetes or Docker Swarm seamlessly integrate with image repositories to deploy and manage containerized applications at scale. Orchestration platforms pull container images from the repository based on deployment specifications, ensuring consistent and reliable deployments across clusters of nodes. Continuous integration and continuous deployment (CI/CD) pipelines can also leverage image repositories to automate the build, test, and deployment of containerized applications.

### **Additional resources**

Container Networking: [https://www.vmware.com/topics/glossary/content/container-networking.html](https://www.vmware.com/topics/glossary/content/container-networking.html)

## **Key Concepts**

After completing this episode, you should be able to:

Discuss key concepts in the area of virtualization

**Description:** In this episode, you will learn about some of the key concepts involved in virtualization, especially the virtualization techniques found in modern cloud environments. Topics include stand alone versus clustered virtualization, cloning, host affinity, and hardware pass-through.

### **Key Concepts**

### **Additional resources**

Emulation, Paravirtualization, and Pass-Through: [https://www.techtarget.com/searchvirtualdesktop/opinion/Emulation-paravirtualization-and-pass-through-what-you-need-to-know](https://www.techtarget.com/searchvirtualdesktop/opinion/Emulation-paravirtualization-and-pass-through-what-you-need-to-know-for-client-hypervisors)

## **Network Types**

After completing this episode, you should be able to:

Discuss network types seen today in virtualization environments

**Description:** In this episode, you will learn about network types often seen in virtualized environments. This includes a discussion of overlay networks and virtual machine (VM) networks.

### **Network Types**

Overlay network - a virtual network that is built on top of an existing physical network infrastructure, enabling the creation of logical connections and the implementation of network services without altering the underlying hardware. This is achieved by encapsulating the overlay network's data packets inside the standard packets of the underlying network, allowing for features such as enhanced security, improved scalability, and efficient resource management. Overlay networks are commonly used in applications such as virtual private networks (VPNs), content delivery networks (CDNs), and peer-to-peer (P2P) networks, providing a flexible and adaptable way to optimize network performance and functionality beyond the capabilities of the base infrastructure.

Virtual machine (VM) networks - specialized network configurations that enable communication between virtual machines within a virtualized environment. These networks are established through software-defined networking (SDN) within a hypervisor, allowing VMs to interact as if they were part of a physical network, despite being hosted on a single or multiple physical servers. VM networks provide isolation, security, and flexibility, supporting various network topologies and facilitating the management of network resources dynamically. They allow for advanced features such as VLANs (Virtual Local Area Networks), which segment traffic to enhance security and efficiency, and virtual network interfaces, which enable VMs to connect to both internal and external networks seamlessly. This setup is crucial in data centers and cloud computing environments, where resource optimization and agility are paramount.

### **Additional resources**

Overlay Networks: [https://www.techtarget.com/searchnetworking/definition/overlay-network](https://www.techtarget.com/searchnetworking/definition/overlay-network)

## **Storage Types**

After completing this episode, you should be able to:

Discuss various storage types used in virtualization environments

**Description:** In this episode, you will learn about some of the various storage types that are found in virtualized environments today. These include local storage, network attached storage (NAS), and storage area network (SAN) type technologies.

### **Storage Types**

Local storage - in a virtualized server environment, local storage refers to the use of physical storage devices, such as hard drives or SSDs, that are directly attached to the physical servers hosting the virtual machines (VMs). This type of storage is leveraged to store VM images, data, and applications, offering high performance and low latency due to its proximity to the compute resources. Local storage is particularly beneficial for workloads that require fast data access and minimal latency, such as databases and high-performance applications. However, it lacks the flexibility and redundancy provided by network-attached storage (NAS) or storage area networks (SANs). To mitigate this, virtualization platforms often incorporate features like storage virtualization and replication to enhance reliability and availability, ensuring that VMs can maintain continuity and performance even if a single physical storage device fails.

Network attached storage (NAS) - in a virtualized server environment, a Network-Attached Storage (NAS) system provides centralized, shared storage accessible over the network, enabling multiple virtual machines (VMs) to store and retrieve data efficiently. NAS offers several advantages, including simplified storage management, enhanced data sharing, and improved scalability, as storage capacity can be easily expanded without disrupting the VMs. This centralized approach also facilitates data backup, disaster recovery, and high availability, ensuring that critical data remains accessible and protected. By decoupling storage from individual physical servers, NAS helps optimize resource utilization and supports dynamic workload management in virtualized environments.

Storage area network (SAN) - in a virtualized server environment, a Storage Area Network (SAN) provides a high-performance, highly available, and scalable storage solution by connecting multiple servers to a centralized pool of storage devices over a dedicated network. SANs are designed for speed and efficiency, supporting demanding applications and large-scale virtual machine (VM) deployments with minimal latency and high throughput. They enable advanced storage management features such as snapshots, replication, and thin provisioning, which enhance data protection, disaster recovery, and resource utilization. By decoupling storage from individual physical servers, SANs facilitate flexible and dynamic allocation of storage resources, improving overall system efficiency and scalability in virtualized infrastructures.

### **Additional resources**

What is a SAN: [https://www.snia.org/education/storage_networking_primer/san/what_san](https://www.snia.org/education/storage_networking_primer/san/what_san)

## **Billing Models**

After completing this episode, you should be able to:

Discuss some of the various billing models commonly found with cloud computing

**Description:** In this episode, you will learn about some of the common billing models found in use in cloud computing today. These models include the pay-as-you-go model, as well as the dedicated host, reserved resources, and spot instance billing models.

### **Billing Models**

Pay As You Go: The pay-as-you-go (PAYG) cloud cost model is a flexible and scalable pricing strategy where users are billed based on their actual consumption of cloud resources, such as computing power, storage, and data transfer. This model eliminates the need for upfront investments and allows for dynamic scaling, making it cost-efficient and adaptable to fluctuating workloads. Users benefit from granular billing, detailed usage monitoring, and the ability to manage costs precisely, encouraging efficient resource utilization. This approach supports business agility, enabling quick deployment and experimentation with minimal financial risk, as seen in services like AWS EC2, Google Compute Engine, and Azure Virtual Machines.

Spot instance: The spot instance cost model in cloud pricing offers users the ability to bid on and use spare computing capacity at significantly reduced rates compared to standard on-demand instances. This model is designed to utilize idle resources in a cloud provider's data center, providing a cost-effective option for users who have flexible and non-urgent workloads. Spot instances can be interrupted by the cloud provider with short notice if the capacity is needed for other users willing to pay higher prices. This makes them ideal for batch processing, data analysis, CI/CD pipelines, and other fault-tolerant tasks that can handle interruptions. While spot instances offer substantial savings—often up to 90% off on-demand prices—they require applications to be resilient and able to manage sudden terminations. This pricing model is used by major cloud providers, such as AWS Spot Instances, Google Cloud Preemptible VMs, and Azure Spot VMs, offering users a cost-efficient way to leverage excess capacity in the cloud.

Reserved resources: The reserved resources cloud pricing model allows users to commit to using specific cloud resources, such as compute instances or storage, for a set period, typically one or three years, in exchange for a significant discount compared to ondemand pricing. By making an upfront payment or committing to a usage term, users benefit from predictable costs and substantial savings, often ranging from 30% to 75% off standard rates. This model is ideal for stable, long-term workloads where resource requirements are predictable, providing financial advantages while ensuring the availability of the reserved capacity. Major cloud providers, including AWS Reserved Instances, Google Cloud Committed Use Contracts, and Azure Reserved VM Instances, offer this model to support cost-effective, reliable resource planning for businesses.

Dedicated host: The dedicated host cloud pricing model provides users with physical servers exclusively allocated to them, offering enhanced control, compliance, and performance benefits. Unlike shared infrastructure, where resources are virtualized across multiple customers, dedicated hosts ensure that all server resources are available solely to the renting organization. This model is particularly advantageous for meeting regulatory requirements, managing software licenses that are tied to physical servers, and optimizing resource allocation without the "noisy neighbor" effect. Typically, dedicated hosts come with a premium price, reflecting the exclusivity and control they offer. Cloud providers such as AWS, Azure, and Google Cloud offer dedicated hosts, enabling businesses to achieve higher security, compliance, and customization tailored to their specific needs.

### **Additional resources**

Cloud Cost: [https://spot.io/resources/cloud-cost/cloud-cost-models-management-strategies/](https://spot.io/resources/cloudcost/cloud-cost-models-management-strategies/)

## **Other Cost Topics**

After completing this episode, you should be able to:

Discuss other cost-related topics

**Description:** In this episode, you will learn about some of the other various cost topics that are typically associated with cloud environments. These topics include resource metering, tagging, and rightsizing.

### **Other Cost Topics**

Resource metering - Resource metering in cloud environments is the process of tracking and measuring the usage of various cloud resources such as computing power, storage, bandwidth, and application services. This data collection is essential for billing, performance monitoring, and capacity planning, providing detailed insights into how resources are being utilized over time. Metering enables cloud providers to implement pay-as-you-go pricing models by accurately charging users based on their actual consumption. It also helps users optimize their resource usage, identify inefficiencies, and manage costs effectively. Comprehensive metering tools and dashboards are typically offered by cloud providers like AWS, Azure, and Google Cloud, allowing users to monitor their usage patterns, set usage alerts, and make informed decisions about scaling and resource allocation.

Tagging - Resource tagging in cloud environments is a powerful management feature that involves assigning metadata, in the form of key-value pairs, to cloud resources like instances, storage buckets, and databases. These tags facilitate better organization, tracking, and automation by enabling users to categorize and filter resources based on attributes such as purpose, owner, environment (e.g., development, testing, production), and cost center. Effective tagging helps in optimizing resource utilization, enhancing security, and simplifying the implementation of policies and permissions. Additionally, it aids in detailed cost management and reporting, as tags can be used to allocate costs accurately across different projects or departments. Major cloud providers, including AWS, Azure, and Google Cloud, support resource tagging, making it an essential practice for efficient cloud infrastructure management.

Rightsizing - Rightsizing in cloud environments refers to the process of optimizing cloud resources to match the actual workload requirements, thereby improving efficiency and reducing costs. This involves analyzing the performance and utilization metrics of resources such as virtual machines, databases, and storage, and then adjusting their configurations—either scaling up or down—to eliminate underutilization or overprovisioning. Rightsizing ensures that the allocated resources are neither too large, incurring unnecessary costs, nor too small, leading to performance bottlenecks. Cloud providers like AWS, Azure, and Google Cloud offer tools and recommendations to assist in rightsizing, helping businesses achieve an optimal balance between cost and performance, and ultimately maximizing the value derived from their cloud investments.

### **Additional resources**

Tagging: [https://www.economize.cloud/glossary/tagging](https://www.economize.cloud/glossary/tagging)

## **Types of Databases**

After completing this episode, you should be able to:

Discuss the different types of databases that are found in cloud environments today

**Description:** In this episode, you will learn about the variety of databases in use in cloud technologies today. This includes a discussion of relational and non-relational databases.

### **Types of Databases**

Relational databases - relational databases are a type of database management system (DBMS) that store data in structured formats using tables, which are composed of rows and columns. Each table, representing a specific entity type, can be linked to others through foreign keys, establishing relationships between different data entities. This model supports SQL (Structured Query Language) for defining, manipulating, and querying data, enabling efficient organization, retrieval, and integrity of data. Relational databases ensure data accuracy and consistency through constraints, such as primary keys, and support transactions to maintain data integrity in multi-user environments. Examples include MySQL, PostgreSQL, and Oracle Database.

NoSQL databases - NoSQL databases are a class of database management systems designed to handle large volumes of unstructured or semi-structured data, providing flexible schemas and scalability beyond the capabilities of traditional relational databases. They eschew the fixed table structure of relational databases, instead offering diverse data models including document, key-value, column-family, and graph formats. NoSQL databases are optimized for distributed systems, enabling horizontal scaling across many servers, which makes them ideal for big data applications and real-time web analytics. They prioritize performance, availability, and fault tolerance, often at the expense of immediate consistency.

Time-based databases - time-based databases, also known as time series databases, are specialized databases optimized for storing, querying, and analyzing time-stamped data points, which are typically generated sequentially over time. These databases are designed to efficiently handle large volumes of time series data, which can come from various sources such as IoT devices, financial transactions, server logs, and monitoring systems. They provide advanced features for time-based queries, such as aggregations, downsampling, and time windowing, which are essential for analyzing trends, patterns, and anomalies over time. Time-based databases often support high write and query performance, retention policies to manage data lifecycle, and seamless integration with data visualization tools.

Graph databases - a graph database is a type of database designed to store and query data modeled as nodes (entities) and edges (relationships), making it ideal for handling complex, interconnected data structures such as social networks, recommendation systems, and fraud detection.

Memory cache databases - memory cache databases are high-performance data storage systems that temporarily hold frequently accessed data in memory (RAM) to accelerate read and write operations, thereby reducing latency and improving application performance. Unlike traditional disk-based databases, memory cache databases prioritize speed and are typically used to store transient data that can be quickly retrieved or updated, such as session information, user profiles, and temporary computational results. They support various caching strategies, such as Least Recently Used (LRU) and time-to-live (TTL) policies, to manage data lifecycle efficiently. Common use cases include web caching, database query results caching, and in-memory data grids. Examples of memory cache databases include Redis, Memcached, and Apache Ignite.

### **Additional resources**

Types of Databases: [https://www.geeksforgeeks.org/types-of-databases/](https://www.geeksforgeeks.org/types-of-databases/)

## **Database Deployment Options**

After completing this episode, you should be able to:

Discuss the two major options when it comes to database deployments and the cloud

**Description:** In this episode, you will learn about the two major options when it comes to database deployments in the cloud. These options are to self-manage a database deployment or to use a provider-managed (managed service) approach.

### **Database Deployment Options**

Self-managed - Using a self-managed database in the cloud involves deploying, configuring, and maintaining a database instance on cloud infrastructure, giving organizations full control over the database environment, including performance tuning, security configurations, and backup strategies. This approach allows for customization to meet specific application requirements and can leverage cloud benefits such as scalability, geographic distribution, and high availability. However, it also demands significant administrative effort, including regular updates, patching, monitoring, and ensuring compliance with data governance policies. While it provides flexibility and control, it can be resource-intensive compared to managed database services where the cloud provider handles much of the operational overhead. Examples include setting up MySQL, PostgreSQL, or MongoDB on cloud platforms like AWS, Azure, or Google Cloud.

Provider-managed - Using a provider-managed database in the cloud involves leveraging a cloud provider's fully managed database services, which handle the deployment, configuration, maintenance, and scaling of the database infrastructure. This option simplifies database management by offloading routine tasks such as backups, patching, monitoring, and high availability configurations to the cloud provider, allowing development teams to focus on application development and business logic. Managed databases offer automated scaling to handle varying workloads, built-in security features, and integrated support for disaster recovery, ensuring robust performance and reliability. Examples include Amazon RDS, Google Cloud SQL, Azure SQL Database, and MongoDB Atlas, which support a wide range of database engines and provide seamless integration with other cloud services.

### **Additional resources**

What are Database Deployment Options: [https://www.oracle.com/database/what-is-database/deployment-options/](https://www.oracle.com/database/what-is-database/deployment-options/)

## **Compute Resources**

After completing this episode, you should be able to:

Discuss the various compute resource options available in the cloud today

**Description:** In this episode, you will learn about the various compute options that are available in the cloud today. This episode includes a discussion of VMs, containers, and serverless compute options.

### **Compute Resources**

Virtual machines (VMs) - cloud-based virtual machines (VMs) are virtualized computing resources provided by cloud service providers that mimic the functionality of physical servers, allowing users to run operating systems and applications as if they were on dedicated hardware. These VMs offer flexibility in terms of configuration, scalability, and resource allocation, enabling users to easily scale up or down based on demand. They are typically billed on a pay-as-you-go basis, providing cost-efficiency by charging only for the resources consumed. Cloud-based VMs can be quickly deployed, managed, and integrated with other cloud services, supporting a wide range of use cases such as application hosting, development and testing environments, and disaster recovery solutions. Examples of cloud-based VM services include Amazon EC2, Google Compute Engine, and Microsoft Azure Virtual Machines.

Containers - cloud-based containers are lightweight, portable computing environments that package applications and their dependencies into a single unit, ensuring consistency across various computing environments. Managed by container orchestration platforms like Kubernetes, these containers run on cloud infrastructure, enabling efficient scaling, deployment, and management of applications. Unlike traditional virtual machines, containers share the host system's OS kernel, making them more resource-efficient and faster to start. They are ideal for microservices architectures, facilitating continuous integration and deployment (CI/CD) by allowing developers to build, test, and deploy applications in isolated environments. Examples of cloud-based container services include AWS Elastic Kubernetes Service (EKS), Google Kubernetes Engine (GKE), and Azure Kubernetes Service (AKS).

Serverless - serverless compute in cloud technology allows developers to build and run applications without managing the underlying infrastructure, as the cloud provider automatically handles the provisioning, scaling, and maintenance of servers. In this model, code is executed in response to events or requests, with resources allocated dynamically, ensuring efficient utilization and billing based only on the actual compute time consumed. This abstraction enables developers to focus solely on writing and deploying code, enhancing productivity and speeding up development cycles. Serverless compute is highly scalable, automatically adjusting to meet demand, and supports various use cases, including web applications, APIs, data processing, and real-time file processing. Prominent examples of serverless compute services include AWS Lambda, Azure Functions, and Google Cloud Functions.

### **Additional resources**

Choosing the Right Compute Option in GCP:  [https://cloud.google.com/blog/products/compute/choosing-the-right-compute-option-in-gcp-a](https://cloud.google.com/blog/products/compute/choosing-the-right-compute-option-in-gcp-a-decision-tree)

## **Orchestration and Workflows**

After completing this episode, you should be able to:

Discuss orchestration and workflows as they exist in cloud environments today

**Description:** In this episode, you will learn about orchestration and workflows as they exist in modern cloud environments. This episode provides a demonstration of one of the built-in orchestration tools of AWS - the CloudFormation tool.

### **Orchestration and Workflows**

Orchestration - Orchestration in the cloud refers to the automated management and coordination of various cloud resources and services to deploy, configure, scale, and manage complex applications and infrastructure. This involves defining the desired state of the system through code or configuration files and using orchestration tools like Kubernetes, Docker Swarm, or AWS ECS to ensure that the actual state matches the desired state. Orchestration abstracts away the underlying complexities of cloud infrastructure, enabling efficient resource utilization, high availability, fault tolerance, and scalability while promoting consistency, repeatability, and automation in application deployment and management processes across distributed environments.

Workflow - In cloud orchestration, a workflow represents a sequence of interconnected tasks or steps that automate and streamline the execution of complex processes or operations within a cloud environment. These workflows define the flow of data, actions, and dependencies between different components or services, orchestrating the deployment, configuration, monitoring, and scaling of cloud resources and applications. Workflow automation tools such as Apache Airflow, AWS Step Functions, or Azure Logic Apps enable developers and operations teams to design, schedule, and manage workflows visually or through code, allowing for the creation of sophisticated automation pipelines. Workflows in cloud orchestration facilitate collaboration, improve efficiency, and ensure consistency in executing tasks across diverse cloud environments, enhancing agility and accelerating time-to-market for cloud-based solutions.

### **Additional resources**

What is Cloud Orchestration: [https://www.vmware.com/topics/glossary/content/cloud-orchestration.html](https://www.vmware.com/topics/glossary/content/cloud-orchestration.html)

## **Networking and Storage**

After completing this episode, you should be able to:

Discuss optimizing workloads through the use of networking and storage innovations in the cloud

**Description:** In this episode, you will learn about the ability to optimize workloads in the cloud through innovations in networking and storage technologies.

### **Networking and Storage**

Network latency - network latency is the time delay experienced in the transmission of data over a network, encompassing the time it takes for a data packet to travel from its source to its destination and back again. This delay can be influenced by various factors including the physical distance between the source and destination, the speed and quality of the transmission medium, the number of intermediary devices like routers and switches, and the overall network congestion. High latency can result in slower data transfer rates and noticeable lag in real-time applications like video conferencing and online gaming, impacting the user experience significantly. Reducing latency involves optimizing the network infrastructure, minimizing the number of hops, and enhancing data routing efficiency.

Throughput - network throughput refers to the rate at which data is successfully transmitted and received over a network in a given time period, typically measured in bits per second (bps). It indicates the actual data transfer performance of a network, as opposed to the theoretical maximum capacity. Throughput is influenced by factors such as network bandwidth, latency, packet loss, and protocol overhead. High throughput is crucial for efficient network performance, enabling faster data exchange and smoother operation of applications and services. Notice that throughput is also used in the storage discipline. In that context, it refers to the rate at which data is successfully written to and read from storage media.

IOPS - IOPS (Input/Output Operations Per Second) is a performance measurement used in IT storage to quantify the number of read and write operations a storage device or system can handle per second. It is a critical metric for assessing the efficiency and speed of storage solutions, particularly in environments with high transaction rates like databases and enterprise applications. Higher IOPS values indicate better performance, enabling quicker data access and processing. Factors affecting IOPS include the type of storage media (e.g., HDDs, SSDs), the workload pattern, and the storage system's configuration and optimization.

### **Additional resources**

What is Network Latency: [https://aws.amazon.com/what-is/latency/](https://aws.amazon.com/what-is/latency/)

## **AI**

After completing this episode, you should be able to:

Discuss AI and the cloud

**Description:** In this episode, you will learn about the exciting area of Artificial Intelligence (AI) and how the cloud has impacted these developments.

### **Artificial Intelligence**

Text recognition - text recognition in AI, also known as Optical Character Recognition (OCR), involves using machine learning models to convert different types of documents, such as scanned paper documents, PDFs, or images, into editable and searchable data. Advanced OCR systems today utilize deep learning techniques to achieve high accuracy in recognizing diverse fonts, languages, and complex layouts, significantly enhancing automation in data entry and document management processes.

Text translation - text translation in AI employs advanced neural network models, such as transformers, to automatically translate text between different languages with high accuracy and fluency. These models, exemplified by systems like Google Translate and DeepL, continually improve through extensive training on diverse multilingual datasets, enabling real-time and contextually nuanced translations.

Visual recognition - visual recognition in AI leverages deep learning, particularly convolutional neural networks (CNNs), to identify and classify objects, scenes, and activities within images and videos with high accuracy. This technology powers applications like facial recognition, autonomous driving, and medical image analysis, continually improving through large-scale annotated datasets and advanced training algorithms.

Sentiment analysis - sentiment analysis in AI uses natural language processing (NLP) and machine learning techniques to automatically detect and interpret the emotional tone of textual data, categorizing it as positive, negative, or neutral. Advanced models analyze context, sarcasm, and nuanced expressions, making sentiment analysis a valuable tool for businesses in understanding customer opinions and market trends.

Voice-to-text - voice-to-text in AI employs advanced speech recognition algorithms and deep learning models to convert spoken language into written text with high accuracy. These systems, exemplified by services like Google's Speech-to-Text and Apple's Siri, support multiple languages and dialects, facilitating applications in transcription, voice commands, and real-time communication.

Text-to-voice - text-to-voice in AI utilizes neural text-to-speech (TTS) models to convert written text into natural-sounding speech, achieving high levels of realism and expressiveness. These advancements enable applications in virtual assistants, audiobooks, and accessibility tools, providing clear and human-like vocal outputs across various languages and accents.

Generative AI - generative AI employs models like Generative Adversarial Networks (GANs) and Transformers to create new, original content, including text, images, and music, by learning patterns from large datasets. This technology underpins applications such as deepfake creation, automated content generation, and creative assistance, pushing the boundaries of machine creativity and innovation.

### **Additional resources**

The Role of AI in Cloud Computing: [https://www.nutanix.com/theforecastbynutanix/technology/ai-in-the-cloud](https://www.nutanix.com/theforecastbynutanix/technology/ai-in-the-cloud)

## **IoT**

After completing this episode, you should be able to:

Discuss the evolving technology of Internet of Things

**Description:** In this episode, you will learn about the evolving technology of the Internet of Things (IoT) and the impact the cloud has had on this exciting new segment of technology.

### **Internet of Things (IoT)**

Sensors - sensors in IoT collect and transmit data on various physical parameters such as temperature, humidity, motion, and light, enabling real-time monitoring and automation. These sensors form the backbone of smart systems in homes, cities, and industries, enhancing efficiency, safety, and decision-making through continuous data streams and connectivity.

Gateways - gateways in IoT serve as critical intermediaries that connect edge devices and sensors to the cloud or central systems, facilitating data aggregation, processing, and secure transmission. They enable seamless communication across diverse protocols and networks, enhancing the efficiency, scalability, and security of IoT ecosystems.

Communication - communication in IoT involves the exchange of data between interconnected devices using various wireless and wired protocols like Wi-Fi, Bluetooth, Zigbee, and MQTT. This connectivity enables real-time data sharing, remote monitoring, and automation, driving the functionality and intelligence of smart environments and systems.

Transmission protocols - transmission protocols in IoT facilitate efficient and reliable communication between devices and networks, ensuring data is transmitted securely and with minimal latency. Common protocols include MQTT, CoAP, and HTTP, each optimized for specific use cases such as low-power devices, real-time updates, or large-scale deployments.

### **Additional resources**

IoT and Cloud Computing: [https://www.geeksforgeeks.org/iot-and-cloud-computing/](https://www.geeksforgeeks.org/iot-and-cloudcomputing/)
