<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>deleteRelatedPatientUsingDELETE</name>
   <tag></tag>
   <elementGuidId>27ca1b92-5288-4fc4-9947-87c035bcd25d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://localhost:9098/clevehr-services/api/v1/patients/patientRelated/${patientId}${/relatedPatient/}${relatedPatientId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>48033c3b-a49b-44b7-88b9-ceae41a958d1</id>
      <masked>false</masked>
      <name>patientId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bb85d69a-7f46-49f5-bde2-929877efad7e</id>
      <masked>false</masked>
      <name>relatedPatientId</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
